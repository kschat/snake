use crate::{
    engine::{
        point::Point,
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

#[derive(Debug)]
pub struct Score {
    value: u32,
    content: String,
    position: Point,
}

impl Score {
    pub fn new(position: Point) -> Self {
        let value = 0;
        Self {
            value,
            content: Self::format_score(value),
            position,
        }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        self.content = Self::format_score(self.value);
    }

    fn format_score(value: u32) -> String {
        format!(" Score: {value} ")
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
