mod engine;
mod entities;
mod snake_scene;

use anyhow::{Context, Result};
use crossterm::terminal;
use engine::{
    game_loop::{GameLoop, GameLoopConfig},
    renderer::Renderer,
};
use snake_scene::SnakeScene;
use std::{io::stdout, time::Duration};

pub enum PlayerInput {
    Up,
    Down,
    Left,
    Right,
    Pause,
}

fn main() -> Result<()> {
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

    game.load_scene(Box::new(SnakeScene::new(rows, columns)));
    game.run()
}
