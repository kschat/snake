use anyhow::Result;
use crossterm::event::{self, KeyCode};
use event::Event;
use std::time::Duration;

use crate::{
    engine::{
        game_loop::GameLoopSignal,
        point::Point,
        renderer::DrawInstruction,
        timestep::Timestep,
        traits::{Entity, GameScene},
    },
    entities::{food::Food, score::Score, snake::Snake, text::Text, world::World},
    PlayerInput, SnakeConfig,
};

const GAME_OVER: &str = "GAME OVER";

const FPS_LABEL: &str = "FPS: ";

#[derive(Debug, Copy, Clone)]
enum SnakeSceneState {
    Playing,
    Paused,
    GameOver,
}

#[derive(Debug)]
pub struct SnakeScene {
    config: SnakeConfig,
    world: World,
    snake: Snake,
    food: Food,
    score: Score,
    game_over_text: Text,
    fps_text: Text,
    state: SnakeSceneState,
}

impl SnakeScene {
    pub fn new(config: SnakeConfig) -> Self {
        let world = Self::create_world(&config);
        let food = Food::new(world.get_random_position());
        let game_over_text = Text::default()
            .with_value(GAME_OVER.into())
            .center(world.get_center_position())
            .hide();

        let fps_text = Text::default()
            .with_value(FPS_LABEL.into())
            .at_position(Point::new(config.columns - (FPS_LABEL.len() + 6), 0))
            .set_visibility(config.show_frame_rate);

        let snake = world.create_snake();

        Self {
            config,
            world,
            food,
            game_over_text,
            fps_text,
            snake,
            state: SnakeSceneState::Playing,
            score: Score::new(Point::new(2, 0)),
        }
    }

    fn create_world(config: &SnakeConfig) -> World {
        World::new(config, Point::new(0, 0))
    }

    fn update_scene(&mut self, elapsed: &Duration) -> Result<GameLoopSignal> {
        self.snake.update(elapsed);

        if self.world.detect_collision(self.snake.head()) || self.snake.detect_self_collision() {
            self.state = SnakeSceneState::GameOver;
            self.game_over_text.visible = true;

            return Ok(GameLoopSignal::Run);
        }

        if self.snake.detect_head_collision(self.food.get_position()) {
            self.snake.grow(self.config.grow_rate);
            self.food = self.spawn_food();
            self.score.increment();
        }

        Ok(GameLoopSignal::Run)
    }

    fn spawn_food(&self) -> Food {
        let mut tries = 0;
        let mut position = self.world.get_random_position();
        while tries < 4 && self.snake.detect_collision(position) {
            tries += 1;
            position = self.world.get_random_position();
        }

        Food::new(position)
    }
}

impl GameScene for SnakeScene {
    fn draw(&mut self, timestep: &Timestep) -> Vec<DrawInstruction> {
        self.fps_text
            .update_value(format!(" {}{} ", FPS_LABEL, timestep.frame_rate));

        vec![
            self.game_over_text.draw(),
            self.food.draw(),
            self.world.draw(),
            self.snake.draw(),
            self.score.draw(),
            self.fps_text.draw(),
        ]
        .into_iter()
        .flatten()
        .collect()
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
            _ => PlayerInput::Noop,
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
