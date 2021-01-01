use crate::{
    engine::{
        point::Point,
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

pub struct Score {
    value: u32,
    content: String,
    position: Point,
}

impl Score {
    pub fn new(position: Point) -> Self {
        Self {
            value: 0,
            content: "Score: 0".into(),
            position,
        }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        self.content = format!("Score: {}", self.value);
    }
}

impl Entity for Score {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Text {
            content: &self.content,
            position: self.position,
            style: Style::default(),
        }]
    }
}
