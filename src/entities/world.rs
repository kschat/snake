use rand::prelude::*;
use std::cell::RefCell;

use crate::{
    engine::{point::Point, renderer::DrawInstruction, traits::Entity},
    PlayerInput,
};

// TODO make world an entity manager
pub struct World {
    rows: usize,
    columns: usize,
    rng: RefCell<ThreadRng>,
}

impl World {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    pub fn detect_collision(&self, point: &Point) -> bool {
        point.x == 0
            || (point.x + 2) >= self.columns - 1
            || point.y == 0
            || point.y >= self.rows - 1
    }

    pub fn get_random_position(&self) -> Point {
        let mut rng = self.rng.borrow_mut();
        Point::new(
            rng.gen_range(1..(self.columns - 1) / 2),
            rng.gen_range(1..self.rows - 1),
        )
    }

    pub fn get_center_position(&self) -> Point {
        Point::new(self.columns / 2, self.rows / 2)
    }
}

impl Entity for World {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod detect_collision {
        use super::*;

        #[test]
        fn it_detects_horizontal_collision() {
            let world = World::new(6, 6);
            assert!(world.detect_collision(&Point::new(2, 0)));
            assert!(world.detect_collision(&Point::new(2, 5)));
        }

        #[test]
        fn it_detects_vertical_collision() {
            let world = World::new(6, 6);
            assert!(world.detect_collision(&Point::new(0, 2)));
            assert!(world.detect_collision(&Point::new(5, 2)));
        }
    }

    #[cfg(test)]
    mod get_center_position {
        use super::*;

        #[test]
        fn it_returns_the_center_even() {
            let world = World::new(6, 6);
            assert_eq!(world.get_center_position(), Point::new(3, 3));
        }

        #[test]
        fn it_returns_the_center_odd() {
            let world = World::new(5, 5);
            assert_eq!(world.get_center_position(), Point::new(2, 2));
        }
    }
}
