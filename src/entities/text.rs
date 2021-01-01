use crate::{
    engine::{
        point::Point,
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

pub struct Text {
    pub value: String,
    pub position: Point,
}

impl Entity for Text {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Text {
            content: &self.value,
            position: self.position,
            style: Style::default(),
        }]
    }
}
