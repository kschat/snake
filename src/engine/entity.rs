use std::time::Duration;

use super::renderer::DrawInstruction;

// TODO handle input mapping in a generic way
#[derive(Debug, Default)]
pub struct GameInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub quit: bool,
    pub pause: bool,
    pub select: bool,
}

pub trait Entity {
    fn draw(&self) -> Vec<DrawInstruction>;
    fn update(&mut self, _elapsed: &Duration) {}
    fn process_input(&mut self, _input: &GameInput) {}
}
