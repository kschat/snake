use crate::{
    engine::{
        entity::Entity,
        point::Point,
        renderer::{DrawInstruction, Style},
    },
    SnakeInput,
};

pub struct Text {
    pub value: String,
    pub position: Point,
}

impl Entity for Text {
    type Input = SnakeInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Text {
            content: &self.value,
            position: self.position,
            style: Style::default(),
        }]
    }
}
