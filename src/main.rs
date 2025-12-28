mod config;
mod engine;
mod entities;
mod scenes;

use anyhow::{Context, Result};
use config::GameConfig;
use crossterm::terminal;
use engine::{
    game_loop::{GameLoop, GameLoopConfig},
    renderer::Renderer,
};
use scenes::{snake::SnakeScene, title::TitleScene};
use std::{
    io::{BufWriter, stdout},
    time::Duration,
};
use structopt::StructOpt;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "snake")]
pub struct CommandOptions {
    #[structopt(
        short,
        long,
        default_value = "15.0",
        help = "Set how many tiles per second the snakes moves"
    )]
    speed: f32,

    #[structopt(
        short,
        long,
        default_value = "2",
        help = "Set the rate at which the snake grows when eating food"
    )]
    grow_rate: usize,

    #[structopt(
        short,
        long,
        default_value = "15",
        help = "Set the max frame rate to target"
    )]
    frame_rate: u8,

    #[structopt(long, help = "Display the current frame rate")]
    show_frame_rate: bool,

    #[structopt(short = "b", long, help = "Wrap the game area in a border")]
    show_border: bool,
}

fn main() -> Result<()> {
    let command_options = CommandOptions::from_args();

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
