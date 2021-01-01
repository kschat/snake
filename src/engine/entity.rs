use std::time::Duration;

use super::renderer::DrawInstruction;

pub trait Entity {
    type Input;

    fn draw(&self) -> Vec<DrawInstruction>;
    fn update(&mut self, _elapsed: &Duration) {}
    fn process_input(&mut self, _input: &Self::Input) {}
}
