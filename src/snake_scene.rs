use anyhow::Result;
use crossterm::event;
use event::Event;
use std::time::Duration;

use crate::{
    engine::{
        entity::{Entity, GameInput},
        game_loop::{GameLoopSignal, GameScene},
        point::Point,
        renderer::DrawInstruction,
    },
    entities::{board::Board, food::Food, score::Score, snake::Snake, text::Text},
};

const GAME_OVER: &'static str = "Game over";

pub struct SnakeScene {
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

        if self.snake.detect_collision(self.food.get_position()) {
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
