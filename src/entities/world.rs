use rand::prelude::*;
use std::cell::RefCell;

use crate::{
    GameConfig, PlayerInput,
    engine::{point::Point, renderer::DrawInstruction, traits::Entity},
};

use super::snake::Snake;

// TODO make world an entity manager
#[derive(Debug)]
pub struct World {
    origin: Point,
    diagonal: Point,
    show_border: bool,
    snake_speed: f32,
    rng: RefCell<ThreadRng>,
}

impl World {
    pub fn new(config: &GameConfig, origin: Point) -> Self {
        let diagonal = Point::new(config.columns - origin.x, config.rows - origin.y);
        Self {
            origin,
            diagonal,
            show_border: config.show_border,
            snake_speed: config.snake.speed,
            rng: RefCell::new(rand::rng()),
        }
    }

    pub fn detect_collision(&self, point: Point) -> bool {
        point.x <= self.origin.x
            || point.x >= self.diagonal.x - 2
            || point.y <= self.origin.y
            || point.y >= self.diagonal.y - 1
    }

    pub fn get_random_position(&self) -> Point {
        let mut rng = self.rng.borrow_mut();
        Point::new(
            rng.random_range((self.origin.x + 1)..((self.diagonal.x - 1) / 2)),
            rng.random_range((self.origin.y + 1)..(self.diagonal.y - 1)),
        )
    }

    pub fn get_center_position(&self) -> Point {
        Point::new(
            (self.origin.x + self.diagonal.x) / 2,
            (self.origin.y + self.diagonal.y) / 2,
        )
    }

    pub fn create_snake(&self) -> Snake {
        Snake::new(self.origin + Point::new(2usize, 2), 6, self.snake_speed)
    }
}

impl Entity for World {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction<'_>> {
        if !self.show_border {
            return vec![];
        }

        vec![DrawInstruction::Rectangle {
            position: self.origin,
            width: self.diagonal.x - self.origin.x,
            height: self.diagonal.y - self.origin.y,
            style: Default::default(),
        }]
    }
}

#[cfg(test)]
mod tests {
    use crate::config::SnakeConfig;

    use super::*;

    const CONFIG: GameConfig = GameConfig {
        snake: SnakeConfig {
            speed: 5.0,
            grow_rate: 1,
        },
        rows: 6,
        columns: 6,
        frame_rate: 15,
        show_frame_rate: false,
        show_border: false,
    };

    #[cfg(test)]
    mod detect_collision {
        use super::*;

        #[test]
        fn it_detects_horizontal_collision() {
            let origin = Point::new(0, 0);
            let world = World::new(&CONFIG, origin);
            assert!(world.detect_collision(Point::new(2, 0)));
            assert!(world.detect_collision(Point::new(2, 5)));
        }

        #[test]
        fn it_detects_vertical_collision() {
            let origin = Point::new(0, 0);
            let world = World::new(&CONFIG, origin);
            assert!(world.detect_collision(Point::new(0, 2)));
            assert!(world.detect_collision(Point::new(5, 2)));
        }
    }

    #[cfg(test)]
    mod get_center_position {
        use super::*;

        #[test]
        fn it_returns_the_center_even() {
            let origin = Point::new(0, 0);
            let world = World::new(&CONFIG, origin);
            assert_eq!(world.get_center_position(), Point::new(3, 3));
        }

        #[test]
        fn it_returns_the_center_odd() {
            let config = {
                let mut c = CONFIG.clone();
                c.rows = 5;
                c.columns = 5;
                c
            };
            let origin = Point::new(0, 0);
            let world = World::new(&config, origin);
            assert_eq!(world.get_center_position(), Point::new(2, 2));
        }
    }
}
