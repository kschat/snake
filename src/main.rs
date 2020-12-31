mod engine;
mod entities;
mod snake_scene;

#[macro_use]
extern crate impl_ops;
extern crate num_traits;

use anyhow::{Context, Result};
use crossterm::terminal;
use engine::{game_loop::GameLoop, renderer::Renderer};
use snake_scene::SnakeScene;
use std::io::stdout;

fn main() -> Result<()> {
    let (columns, rows) =
        terminal::size().with_context(|| format!("Failed to get terminal size"))?;
    let columns = (columns - 1) as usize;
    let rows = (rows - 1) as usize;

    let renderer = Renderer::new(stdout(), rows, columns);
    let mut game = GameLoop::new(renderer, 15);

    game.load_scene(Box::new(SnakeScene::new(rows, columns)));
    game.run()
}
