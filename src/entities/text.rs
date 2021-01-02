use crate::{
    engine::{
        point::Point,
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

#[derive(Default)]
pub struct Text {
    pub value: String,
    pub position: Point,
    pub visible: bool,
}

impl Entity for Text {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        if !self.visible {
            return vec![];
        }

        vec![DrawInstruction::Text {
            content: &self.value,
            position: self.position,
            style: Style::default(),
        }]
    }
}
