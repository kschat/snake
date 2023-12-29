use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode},
    style::Color,
};
use std::time::Duration;

use crate::{
    engine::{
        game_loop::GameLoopSignal,
        point::Point,
        renderer::DrawInstruction,
        timestep::Timestep,
        traits::{Entity, GameScene},
    },
    entities::text::Text,
    snake_scene::SnakeScene,
    SnakeConfig,
};

const TITLE: &str = "
███████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗
██╔════╝████╗  ██║██╔══██╗██║ ██╔╝██╔════╝
███████╗██╔██╗ ██║███████║█████╔╝ █████╗
╚════██║██║╚██╗██║██╔══██║██╔═██╗ ██╔══╝
███████║██║ ╚████║██║  ██║██║  ██╗███████╗
╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝
";

#[derive(Debug)]
pub struct TitleScene {
    // config: SnakeConfig,
    // origin: Point,
    // diagonal: Point,
    title_text: Text,
    new_game_text: Text,
    settings_text: Text,
}

impl TitleScene {
    pub fn new(config: SnakeConfig) -> Self {
        let origin = Point::new(0, 0);
        let diagonal = Point::new(config.columns - origin.x, config.rows - origin.y);
        let center = Self::get_center_position(origin, diagonal);
        eprintln!("CONFIG {config:?}");
        eprintln!("ORIGIN {origin:?}");
        eprintln!("DIAGONAL {diagonal:?}");

        let title_text = Text::default()
            .with_value(TITLE.into())
            .center(center - Point::new(0usize, 15))
            .with_fg(Color::Yellow)
            .show();

        let new_game_text = Text::default()
            .with_value("    NEW GAME    ".into())
            .center(center - Point::new(0usize, 5))
            .with_fg(Color::Black)
            .with_bg(Color::Yellow)
            .show();

        let settings_text = Text::default()
            .with_value("    SETTINGS    ".into())
            .center(center - Point::new(0usize, 4))
            .with_fg(Color::Yellow)
            .with_bg(Color::Red)
            .show();

        Self {
            // config,
            // origin,
            // diagonal,
            title_text,
            new_game_text,
            settings_text,
        }
    }

    pub fn get_center_position(origin: Point, diagonal: Point) -> Point {
        Point::new((origin.x + diagonal.x) / 2, (origin.y + diagonal.y) / 2)
    }
}

impl GameScene for TitleScene {
    fn draw(&mut self, _timestep: &Timestep) -> Vec<DrawInstruction> {
        vec![
            self.title_text.draw(),
            self.new_game_text.draw(),
            self.settings_text.draw(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn update(&mut self, _elapsed: &Duration) -> Result<GameLoopSignal> {
        Ok(GameLoopSignal::Run)
    }

    fn process_input(&mut self, event: &crossterm::event::Event) -> Result<GameLoopSignal> {
        let signal = match event {
            Event::Key(e) => match e.code {
                KeyCode::Char('s') => GameLoopSignal::load_scene::<SnakeScene>(),
                KeyCode::Char('q') => GameLoopSignal::Stop,
                _ => GameLoopSignal::Run,
            },
            _ => GameLoopSignal::Run,
        };
        Ok(signal)
    }
}
