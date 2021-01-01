use anyhow::Result;
use crossterm::event::{self, KeyCode};
use event::Event;
use std::time::Duration;

use crate::{
    engine::{
        entity::Entity,
        game_loop::{GameLoopSignal, GameScene},
        point::Point,
        renderer::DrawInstruction,
    },
    entities::{board::Board, food::Food, score::Score, snake::Snake, text::Text},
    SnakeInput,
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
        let input = match event {
            Event::Key(e) => match e.code {
                KeyCode::Char('a') => SnakeInput::Left,
                KeyCode::Char('s') => SnakeInput::Down,
                KeyCode::Char('d') => SnakeInput::Right,
                KeyCode::Char('w') => SnakeInput::Up,
                KeyCode::Char('p') => SnakeInput::Pause,
                KeyCode::Char('q') => return Ok(GameLoopSignal::Stop),
                _ => return Ok(GameLoopSignal::Run),
            },
            Event::Mouse(_) | Event::Resize(_, _) => return Ok(GameLoopSignal::Run),
        };

        if self.game_over {
            return Ok(GameLoopSignal::Pause);
        }

        self.snake.process_input(&input);

        Ok(GameLoopSignal::Run)
    }
}
