mod engine;
mod entities;
mod snake_scene;

use anyhow::{Context, Result};
use crossterm::terminal;
use engine::{
    game_loop::{GameLoop, GameLoopConfig},
    renderer::Renderer,
};
use snake_scene::{SnakeConfig, SnakeScene};
use std::{
    io::{stdout, BufWriter},
    time::Duration,
};
use structopt::StructOpt;

pub enum PlayerInput {
    Up,
    Down,
    Left,
    Right,
    Pause,
    Noop,
    Quit,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "snake")]
struct CommandOptions {
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

fn main() -> Result<()> {
    let command_options = CommandOptions::from_args();

    let (columns, rows) =
        terminal::size().with_context(|| "Failed to get terminal size".to_string())?;
    let columns = columns as usize;
    let rows = rows as usize;

    let mut game = GameLoop::new(
        Renderer::new(BufWriter::new(stdout()), rows, columns),
        GameLoopConfig {
            frame_rate: command_options.frame_rate,
            input_poll_rate: Duration::from_millis(0),
        },
    );

    game.load_scene(Box::new(SnakeScene::new(SnakeConfig {
        columns,
        rows,
        grow_rate: command_options.grow_rate,
        speed: command_options.speed,
        show_frame_rate: command_options.show_frame_rate,
        show_border: command_options.show_border,
    })));

    game.run()
}
