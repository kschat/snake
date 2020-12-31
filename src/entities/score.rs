use crate::engine::{
    entity::Entity,
    point::Point,
    renderer::{DrawInstruction, Style},
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
    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Text {
            content: &self.content,
            position: self.position,
            style: Style::default(),
        }]
    }
}
