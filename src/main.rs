mod engine;
mod entities;
mod snake_scene;
mod title_scene;

use anyhow::{Context, Result};
use crossterm::terminal;
use engine::{
    game_loop::{GameLoop, GameLoopConfig},
    renderer::Renderer,
};
use snake_scene::SnakeScene;
use std::{
    io::{stdout, BufWriter},
    time::Duration,
};
use structopt::StructOpt;
use title_scene::TitleScene;

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

    #[structopt(short, long, help = "Wrap the game area in a border")]
    show_border: bool,
}

#[derive(Debug, Clone)]
pub struct SnakeConfig {
    pub rows: usize,
    pub columns: usize,
    pub speed: f32,
    pub grow_rate: usize,
    pub show_frame_rate: bool,
    pub show_border: bool,
    pub frame_rate: u8,
}

impl SnakeConfig {
    pub fn new(command_options: CommandOptions, (columns, rows): (u16, u16)) -> Self {
        Self {
            columns: columns as usize,
            rows: rows as usize,
            grow_rate: command_options.grow_rate,
            speed: command_options.speed,
            show_frame_rate: command_options.show_frame_rate,
            show_border: command_options.show_border,
            frame_rate: command_options.frame_rate,
        }
    }
}

fn main() -> Result<()> {
    let command_options = CommandOptions::from_args();

    let terminal_size =
        terminal::size().with_context(|| "Failed to get terminal size".to_string())?;

    let snake_config = SnakeConfig::new(command_options, terminal_size);

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
