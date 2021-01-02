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
use std::{io::stdout, time::Duration};
use structopt::StructOpt;

pub enum PlayerInput {
    Up,
    Down,
    Left,
    Right,
    Pause,
}

// speed, grow rate

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
}

fn main() -> Result<()> {
    let command_options = CommandOptions::from_args();

    let (columns, rows) =
        terminal::size().with_context(|| format!("Failed to get terminal size"))?;
    let columns = (columns - 1) as usize;
    let rows = (rows - 1) as usize;

    let mut game = GameLoop::new(
        Renderer::new(stdout(), rows, columns),
        GameLoopConfig {
            frame_rate: 15,
            input_poll_rate: Duration::from_millis(0),
        },
    );

    game.load_scene(Box::new(SnakeScene::new(SnakeConfig {
        columns,
        rows,
        grow_rate: command_options.grow_rate,
        speed: command_options.speed,
    })));

    game.run()
}
