mod config;
mod engine;
mod entities;
mod scenes;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use config::GameConfig;
use crossterm::{style::Color, terminal};
use engine::{
    game_loop::{GameLoop, GameLoopConfig},
    renderer::Renderer,
};
use scenes::{snake::SnakeScene, title::TitleScene};
use std::{
    io::{BufWriter, stdout},
    time::Duration,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerInput {
    Up,
    Down,
    Left,
    Right,
    Pause,
    Select,
    Noop,
    Quit,
}

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct CommandOptions {
    #[arg(
        short,
        long,
        default_value_t = 15.0,
        help = "Set how many tiles per second the snakes moves"
    )]
    speed: f32,

    #[arg(
        short,
        long,
        default_value_t = 2,
        help = "Set the rate at which the snake grows when eating food"
    )]
    grow_rate: usize,

    #[arg(
        value_enum,
        long,
        default_value_t = SnakeStyle::Green,
        help = "Set style of the snake"
    )]
    snake_style: SnakeStyle,

    #[arg(
        short,
        long,
        default_value_t = 15,
        help = "Set the max frame rate to target"
    )]
    frame_rate: u8,

    #[arg(long, help = "Display the current frame rate")]
    show_frame_rate: bool,

    #[arg(short = 'b', long, help = "Wrap the game area in a border")]
    show_border: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum, Eq, PartialEq)]
pub enum SnakeStyle {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
    Flash,
}

impl SnakeStyle {
    pub fn initial_color(&self) -> Color {
        match self {
            Self::Flash | Self::Green => Color::Green,
            Self::Black => Color::Black,
            Self::Red => Color::Red,
            Self::Yellow => Color::Yellow,
            Self::Blue => Color::Blue,
            Self::Magenta => Color::Magenta,
            Self::Cyan => Color::Cyan,
            Self::White => Color::White,
            Self::Grey => Color::Grey,
        }
    }
}

fn main() -> Result<()> {
    let command_options = CommandOptions::parse();

    let terminal_size =
        terminal::size().with_context(|| "Failed to get terminal size".to_string())?;

    let snake_config = GameConfig::new(command_options, terminal_size);

    let mut game_loop = GameLoop::new(
        Renderer::new(
            BufWriter::new(stdout()),
            snake_config.rows,
            snake_config.columns,
        ),
        GameLoopConfig {
            frame_rate: snake_config.frame_rate,
            input_poll_rate: Duration::from_millis(0),
        },
    );

    game_loop
        .register_scene(TitleScene::new(snake_config.clone()))
        .register_scene(SnakeScene::new(snake_config.clone()))
        .run::<TitleScene>()
}
