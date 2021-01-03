use crossterm::style::Color;

use crate::{
    engine::{
        point::Point,
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

pub struct Food {
    position: Point,
}

impl Food {
    pub fn new(position: Point) -> Self {
        Self {
            position: position * Point::new(2, 1),
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }
}

impl Entity for Food {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        vec![DrawInstruction::Square {
            size: 1,
            position: self.position,
            style: Style {
                fg: Color::Red,
                ..Style::default()
            },
        }]
    }
}
