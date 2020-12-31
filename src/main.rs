#[macro_use]
extern crate impl_ops;
extern crate num_traits;

use std::{cell::RefCell, io::stdout, iter::repeat_with, time::Duration};

use anyhow::{Context, Result};
use event::Event;
use rand::{prelude::ThreadRng, Rng};

use crossterm::{event, style::Color, terminal};

use engine::{
    game_loop::{GameLoop, GameLoopSignal, GameScene},
    point::{AbsPoint, Point},
    renderer::{DrawInstruction, Renderer, Style},
};

mod engine;

const ACC: f32 = 15.0;
const GAME_OVER: &'static str = "Game over";

trait Entity {
    fn draw(&self) -> Vec<DrawInstruction>;
    fn update(&mut self, _elapsed: &Duration) {}
    fn process_input(&mut self, _input: &GameInput) {}
}

struct Text {
    value: String,
    position: Point,
}

impl Entity for Text {
    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Text {
            content: &self.value,
            position: self.position,
            style: Style::default(),
        }]
    }
}

struct Score {
    value: u32,
    content: String,
    position: Point,
}

impl Score {
    pub fn new(position: Point) -> Self {
        Self {
            value: 0,
            content: "Score: 0".into(),
            position,
        }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        self.content = format!("Score: {}", self.value);
    }
}

impl Entity for Score {
    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Text {
            content: &self.content,
            position: self.position,
            style: Style::default(),
        }]
    }
}

struct Food {
    position: Point,
}

impl Food {
    pub fn new(position: Point) -> Self {
        Self {
            position: position * Point::new(2.0, 1.0),
        }
    }
}

impl Entity for Food {
    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Square {
            size: 1,
            position: self.position,
            style: Style {
                fg: Color::Red,
                ..Style::default()
            },
        }]
    }
}

struct Snake {
    body: Vec<Point>,
    size: usize,
    velocity: Point,
}

impl Snake {
    pub fn new(head: Point, size: usize) -> Self {
        let body = repeat_with(|| head)
            .enumerate()
            .map(|(index, point)| point + Point::new(((size - index) * 2) as f32, 0.0))
            .take(size)
            .collect();

        Self {
            body,
            size,
            velocity: Point::new(1.0, 0.0),
        }
    }

    pub fn head(&self) -> &Point {
        &self.body[0]
    }

    pub fn detect_collision(&self, point: &Point) -> bool {
        let head = AbsPoint::from(self.body[0]);
        head == AbsPoint::from(*point)
    }

    pub fn self_collision(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .find(|&part| self.detect_collision(part))
            .is_some()
    }

    pub fn grow(&mut self, amount: usize) {
        self.size += amount;
    }
}

impl Entity for Snake {
    fn draw(&self) -> Vec<DrawInstruction> {
        self.body
            .iter()
            .map(|&position| DrawInstruction::Square {
                size: 1,
                position,
                style: Style {
                    fg: Color::Green,
                    ..Style::default()
                },
            })
            .collect()
    }

    fn update(&mut self, elapsed: &Duration) {
        let elapsed_secs = elapsed.as_secs_f32();
        let velocity = self.velocity * (ACC * elapsed_secs);

        let head = self.body[0];
        let new_head = AbsPoint::from(head + velocity);
        let abs_head = AbsPoint::from(head);

        if abs_head.x != new_head.x || abs_head.y != new_head.y {
            if self.size != self.body.len() {
                self.body.insert(0, head);
            } else {
                self.body.rotate_right(1);
            }
        }

        // "squares" are 2x1 since fonts are taller than they are wide so we need a transform
        // if we're moving east or west so we move 2 "pixels" at a time
        let transform = match (abs_head.x < new_head.x, abs_head.x > new_head.x) {
            (true, _) => Point::new(1.0, 0.0),
            (_, true) => Point::new(-1.0, 0.0),
            (_, _) => Point::new(0.0, 0.0),
        };

        self.body[0] = transform + head + velocity;
    }

    fn process_input(&mut self, input: &GameInput) {
        if input.up && self.velocity.y == 0.0 {
            self.velocity = Point::new(0.0, -1.0);
        }

        if input.down && self.velocity.y == 0.0 {
            self.velocity = Point::new(0.0, 1.0);
        }

        if input.right && self.velocity.x == 0.0 {
            self.velocity = Point::new(1.0, 0.0);
        }

        if input.left && self.velocity.x == 0.0 {
            self.velocity = Point::new(-1.0, 0.0);
        }
    }
}

struct Board {
    rows: usize,
    columns: usize,
    rng: RefCell<ThreadRng>,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    pub fn detect_collision(&self, point: &Point) -> bool {
        let point = AbsPoint::from(*point);
        point.x <= 0
            || (point.x + 2) >= self.columns - 1
            || point.y <= 0
            || point.y >= self.rows - 1
    }

    pub fn get_random_position(&self) -> Point {
        let mut rng = self.rng.borrow_mut();
        Point::new(
            rng.gen_range(1..(self.columns - 1) / 2) as f32,
            rng.gen_range(1..self.rows - 1) as f32,
        )
    }

    pub fn get_center_position(&self) -> Point {
        Point::new((self.columns / 2) as f32, (self.rows / 2) as f32)
    }
}

impl Entity for Board {
    fn draw(&self) -> Vec<DrawInstruction> {
        vec![]
    }
}

struct SnakeScene {
    board: Board,
    snake: Snake,
    food: Food,
    score: Score,
    game_over_text: Text,
    game_over: bool,
}

impl SnakeScene {
    pub fn new(rows: usize, columns: usize) -> Self {
        let board = Board::new(rows, columns);
        let food = Food::new(board.get_random_position());
        let game_over_text = Text {
            value: "".into(),
            position: board.get_center_position() - Point::new((GAME_OVER.len() / 2) as f32, 0.0),
        };

        Self {
            board,
            food,
            game_over_text,
            game_over: false,
            score: Score::new(Point::new(0.0, 0.0)),
            snake: Snake::new(Point::new(4.0, 2.0), 6),
        }
    }
}

impl GameScene for SnakeScene {
    fn draw<'a>(&'a mut self) -> Vec<DrawInstruction<'a>> {
        vec![
            self.score.draw(),
            self.game_over_text.draw(),
            self.food.draw(),
            self.snake.draw(),
            self.board.draw(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }

    fn update(&mut self, elapsed: &Duration) -> Result<GameLoopSignal> {
        self.snake.update(elapsed);

        if self.board.detect_collision(&self.snake.head()) {
            self.game_over = true;
            self.game_over_text.value = GAME_OVER.into();

            return Ok(GameLoopSignal::Pause);
        }

        if self.snake.self_collision() {
            self.game_over = true;
            self.game_over_text.value = GAME_OVER.into();

            return Ok(GameLoopSignal::Pause);
        }

        if self.snake.detect_collision(&self.food.position) {
            self.snake.grow(2);
            self.food = Food::new(self.board.get_random_position());
            self.score.increment();
        }

        Ok(GameLoopSignal::Run)
    }

    fn process_input(&mut self, event: &Event) -> Result<GameLoopSignal> {
        let mut input = GameInput::default();
        match event {
            event::Event::Key(e) => match e.code {
                event::KeyCode::Char('a') => input.left = true,
                event::KeyCode::Char('s') => input.down = true,
                event::KeyCode::Char('d') => input.right = true,
                event::KeyCode::Char('w') => input.up = true,
                event::KeyCode::Char('q') => input.quit = true,
                event::KeyCode::Char('p') => input.pause = true,
                event::KeyCode::Enter => input.select = true,
                _ => (),
            },
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        };

        if input.quit {
            return Ok(GameLoopSignal::Stop);
        }

        if self.game_over {
            return Ok(GameLoopSignal::Pause);
        }

        self.snake.process_input(&input);

        Ok(GameLoopSignal::Run)
    }
}

#[derive(Debug, Default)]
struct GameInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    quit: bool,
    pause: bool,
    select: bool,
}

fn main() -> Result<()> {
    let (columns, rows) =
        terminal::size().with_context(|| format!("Failed to get terminal size"))?;
    let columns = (columns - 1) as usize;
    let rows = (rows - 1) as usize;

    let renderer = Renderer::new(stdout(), rows, columns);
    let mut game = GameLoop::new(renderer, 15);
    game.load_scene(Box::new(SnakeScene::new(rows, columns)));

    game.run()
}
