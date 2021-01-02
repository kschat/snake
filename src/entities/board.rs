use rand::prelude::*;
use std::cell::RefCell;

use crate::{
    engine::{
        point::{AbsPoint, Point},
        renderer::DrawInstruction,
        traits::Entity,
    },
    PlayerInput,
};

pub struct Board {
    rows: usize,
    columns: usize,
    rng: RefCell<ThreadRng>,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    pub fn detect_collision(&self, point: &Point) -> bool {
        let point = AbsPoint::from(point);
        point.x <= 0
            || (point.x + 2) >= self.columns - 1
            || point.y <= 0
            || point.y >= self.rows - 1
    }

    pub fn get_random_position(&self) -> Point {
        let mut rng = self.rng.borrow_mut();
        Point::new(
            rng.gen_range(1..(self.columns - 1) / 2) as f32,
            rng.gen_range(1..self.rows - 1) as f32,
        )
    }

    pub fn get_center_position(&self) -> Point {
        Point::new((self.columns / 2) as f32, (self.rows / 2) as f32)
    }
}

impl Entity for Board {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        vec![]
    }
}
