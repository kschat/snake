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

enum SnakeSceneState {
    Playing,
    Paused,
    GameOver,
}

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
    state: SnakeSceneState,
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
            state: SnakeSceneState::Playing,
            score: Score::new(Point::new(0.0, 0.0)),
            snake,
        }
    }

    fn update_scene(&mut self, elapsed: &Duration) -> Result<GameLoopSignal> {
        self.snake.update(elapsed);

        if self.world.detect_collision(&self.snake.head()) || self.snake.self_collision() {
            self.state = SnakeSceneState::GameOver;
            self.game_over_text.visible = true;

            return Ok(GameLoopSignal::Run);
        }

        if self.snake.detect_collision(self.food.get_position()) {
            self.snake.grow(self.config.grow_rate);
            self.food = Food::new(self.world.get_random_position());
            self.score.increment();
        }

        Ok(GameLoopSignal::Run)
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
        Ok(match self.state {
            SnakeSceneState::Paused | SnakeSceneState::GameOver => GameLoopSignal::Run,
            SnakeSceneState::Playing => self.update_scene(elapsed)?,
        })
    }

    fn process_input(&mut self, event: &Event) -> Result<GameLoopSignal> {
        let input = match event {
            Event::Key(e) => match e.code {
                KeyCode::Char('a') | KeyCode::Left => PlayerInput::Left,
                KeyCode::Char('s') | KeyCode::Down => PlayerInput::Down,
                KeyCode::Char('d') | KeyCode::Right => PlayerInput::Right,
                KeyCode::Char('w') | KeyCode::Up => PlayerInput::Up,
                KeyCode::Char('p') => PlayerInput::Pause,
                KeyCode::Char('q') => PlayerInput::Quit,
                _ => PlayerInput::Noop,
            },
            Event::Mouse(_) | Event::Resize(_, _) => PlayerInput::Noop,
        };

        Ok(match (input, &self.state) {
            (PlayerInput::Quit, _) => GameLoopSignal::Stop,
            (PlayerInput::Pause, SnakeSceneState::Paused) => {
                self.state = SnakeSceneState::Playing;
                GameLoopSignal::Run
            }
            (PlayerInput::Pause, SnakeSceneState::Playing) => {
                self.state = SnakeSceneState::Paused;
                GameLoopSignal::Run
            }
            (input, SnakeSceneState::Playing) => {
                self.snake.process_input(&input);
                GameLoopSignal::Run
            }
            _ => GameLoopSignal::Run,
        })
    }
}
