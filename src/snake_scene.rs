use anyhow::Result;
use crossterm::event::{self, KeyCode};
use event::Event;
use std::time::Duration;

use crate::{
    engine::{
        game_loop::GameLoopSignal,
        point::Point,
        renderer::DrawInstruction,
        traits::{Entity, GameScene},
    },
    entities::{food::Food, score::Score, snake::Snake, text::Text, world::World},
    PlayerInput,
};

const GAME_OVER: &'static str = "Game over";

pub struct SnakeConfig {
    pub rows: usize,
    pub columns: usize,
    pub speed: f32,
    pub grow_rate: usize,
}

pub struct SnakeScene {
    config: SnakeConfig,
    world: World,
    snake: Snake,
    food: Food,
    score: Score,
    game_over_text: Text,
    game_over: bool,
}

impl SnakeScene {
    pub fn new(config: SnakeConfig) -> Self {
        let world = World::new(config.rows, config.columns);
        let food = Food::new(world.get_random_position());
        let game_over_text = Text {
            value: GAME_OVER.into(),
            position: world.get_center_position() - Point::new((GAME_OVER.len() / 2) as f32, 0.0),
            visible: false,
        };

        let snake = Snake::new(Point::new(4.0, 2.0), 6, config.speed);

        Self {
            config,
            world,
            food,
            game_over_text,
            game_over: false,
            score: Score::new(Point::new(0.0, 0.0)),
            snake,
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
            self.world.draw(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }

    fn update(&mut self, elapsed: &Duration) -> Result<GameLoopSignal> {
        self.snake.update(elapsed);

        if self.world.detect_collision(&self.snake.head()) || self.snake.self_collision() {
            self.game_over = true;
            self.game_over_text.visible = true;

            return Ok(GameLoopSignal::Pause);
        }

        if self.snake.detect_collision(self.food.get_position()) {
            self.snake.grow(self.config.grow_rate);
            self.food = Food::new(self.world.get_random_position());
            self.score.increment();
        }

        Ok(GameLoopSignal::Run)
    }

    fn process_input(&mut self, event: &Event) -> Result<GameLoopSignal> {
        let input = match event {
            Event::Key(e) => match e.code {
                KeyCode::Char('a') | KeyCode::Left => PlayerInput::Left,
                KeyCode::Char('s') | KeyCode::Down => PlayerInput::Down,
                KeyCode::Char('d') | KeyCode::Right => PlayerInput::Right,
                KeyCode::Char('w') | KeyCode::Up => PlayerInput::Up,
                KeyCode::Char('p') => PlayerInput::Pause,
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
