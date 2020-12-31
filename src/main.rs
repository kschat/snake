#[macro_use]
extern crate impl_ops;
extern crate num_traits;

use std::{cell::RefCell, io::stdout, iter::repeat_with, thread::sleep, time::Duration};

use anyhow::{Context, Result};
use point::{AbsPoint, Point};
use rand::{prelude::ThreadRng, Rng};

use crossterm::{event, style::Color, terminal};
use renderer::{DrawInstruction, Renderer, Style};
use timestep::Timestep;

mod point;
mod renderer;
mod timestep;

// 1 update every 33 ms = 30 FPS
const MS_PER_UPDATE: Duration = Duration::from_millis(16);
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

fn process_input() -> Result<GameInput> {
    let mut input = GameInput::default();

    if event::poll(Duration::from_millis(0))? {
        let event = event::read()?;

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
        }
    }

    Ok(input)
}

fn game_loop() -> Result<()> {
    let (width, height) =
        terminal::size().with_context(|| format!("Failed to get terminal size"))?;

    let width = (width - 1) as usize;
    let height = (height - 1) as usize;
    let mut renderer = Renderer::new(stdout(), height, width);

    let mut timestep = Timestep::new();
    let mut delta = Duration::from_millis(0);
    let mut game_over = false;

    let board = Board::new(height, width);

    let mut score = Score::new(Point::new(0.0, 0.0));

    let mut game_over_text = Text {
        value: "".into(),
        position: board.get_center_position() - Point::new((GAME_OVER.len() / 2) as f32, 0.0),
    };

    let mut food = Food::new(board.get_random_position());

    let mut snake = Snake::new(Point::new(4.0, 2.0), 6);

    renderer.start()?;

    loop {
        let input = process_input()?;

        if input.quit {
            break;
        }

        delta += timestep.delta();
        while delta >= MS_PER_UPDATE {
            delta -= MS_PER_UPDATE;
            if game_over {
                continue;
            }

            snake.process_input(&input);
            snake.update(&MS_PER_UPDATE);

            if board.detect_collision(&snake.head()) {
                game_over = true;
                game_over_text.value = GAME_OVER.into();
            }

            if snake.detect_collision(&food.position) {
                snake.grow(2);
                food = Food::new(board.get_random_position());
                score.increment();
            }

            if snake.self_collision() {
                game_over = true;
                game_over_text.value = GAME_OVER.into();
            }
        }

        let instructions = &mut vec![
            score.draw(),
            game_over_text.draw(),
            food.draw(),
            snake.draw(),
            board.draw(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        renderer.draw(instructions)?;

        let frame_time = timestep.elapsed_time();
        if frame_time < MS_PER_UPDATE {
            sleep(MS_PER_UPDATE - frame_time);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    game_loop()
}
