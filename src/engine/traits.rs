use anyhow::Result;
use crossterm::event::Event;
use std::time::Duration;

use super::{game_loop::GameLoopSignal, renderer::DrawInstruction, timestep::Timestep};

pub trait Entity {
    type Input;

    fn draw(&self) -> Vec<DrawInstruction>;
    fn update(&mut self, _elapsed: &Duration) {}
    fn process_input(&mut self, _input: &Self::Input) {}
}

pub trait GameScene {
    fn draw(&mut self, timestep: &Timestep) -> Vec<DrawInstruction>;
    fn update(&mut self, elapsed: &Duration) -> Result<GameLoopSignal>;
    fn process_input(&mut self, event: &Event) -> Result<GameLoopSignal>;
}
