use crossterm::style::Color;

use crate::{
    engine::{
        entity::Entity,
        point::Point,
        renderer::{DrawInstruction, Style},
    },
    SnakeInput,
};

pub struct Food {
    position: Point,
}

impl Food {
    pub fn new(position: Point) -> Self {
        Self {
            position: position * Point::new(2.0, 1.0),
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }
}

impl Entity for Food {
    type Input = SnakeInput;

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
