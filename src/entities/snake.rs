use crossterm::style::Color;
use std::{iter::repeat_with, time::Duration};

use crate::{
    engine::{
        point::{AbsPoint, Point},
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

const ACCELERATION: f32 = 15.0;

pub struct Snake {
    body: Vec<Point>,
    size: usize,
    velocity: Point,
}

impl Snake {
    pub fn new(head: Point, size: usize) -> Self {
        let body = repeat_with(|| head)
            .enumerate()
            .map(|(index, point)| point + Point::new(((size - index) * 2) as f32, 0.0))
            .take(size)
            .collect();

        Self {
            body,
            size,
            velocity: Point::new(1.0, 0.0),
        }
    }

    pub fn head(&self) -> &Point {
        &self.body[0]
    }

    pub fn detect_collision(&self, point: &Point) -> bool {
        let head = AbsPoint::from(self.body[0]);
        head == AbsPoint::from(*point)
    }

    pub fn self_collision(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .find(|&part| self.detect_collision(part))
            .is_some()
    }

    pub fn grow(&mut self, amount: usize) {
        self.size += amount;
    }
}

impl Entity for Snake {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction> {
        self.body
            .iter()
            .map(|&position| DrawInstruction::Square {
                size: 1,
                position,
                style: Style {
                    fg: Color::Green,
                    ..Style::default()
                },
            })
            .collect()
    }

    fn update(&mut self, elapsed: &Duration) {
        let elapsed_secs = elapsed.as_secs_f32();
        let velocity = self.velocity * (ACCELERATION * elapsed_secs);

        let head = self.body[0];
        let new_head = AbsPoint::from(head + velocity);
        let abs_head = AbsPoint::from(head);

        if abs_head.x != new_head.x || abs_head.y != new_head.y {
            if self.size != self.body.len() {
                self.body.insert(0, head);
            } else {
                self.body.rotate_right(1);
            }
        }

        // "squares" are 2x1 since fonts are taller than they are wide so we need a transform
        // if we're moving east or west so we move 2 "pixels" at a time
        let transform = match (abs_head.x < new_head.x, abs_head.x > new_head.x) {
            (true, _) => Point::new(1.0, 0.0),
            (_, true) => Point::new(-1.0, 0.0),
            (_, _) => Point::new(0.0, 0.0),
        };

        self.body[0] = transform + head + velocity;
    }

    fn process_input(&mut self, input: &Self::Input) {
        self.velocity = match input {
            PlayerInput::Up if self.velocity.y == 0.0 => Point::new(0.0, -1.0),
            PlayerInput::Down if self.velocity.y == 0.0 => Point::new(0.0, 1.0),
            PlayerInput::Right if self.velocity.x == 0.0 => Point::new(1.0, 0.0),
            PlayerInput::Left if self.velocity.x == 0.0 => Point::new(-1.0, 0.0),
            _ => self.velocity,
        };

        // if input.up && self.velocity.y == 0.0 {
        //     self.velocity = Point::new(0.0, -1.0);
        // }

        // if input.down && self.velocity.y == 0.0 {
        //     self.velocity = Point::new(0.0, 1.0);
        // }

        // if input.right && self.velocity.x == 0.0 {
        //     self.velocity = Point::new(1.0, 0.0);
        // }

        // if input.left && self.velocity.x == 0.0 {
        //     self.velocity = Point::new(-1.0, 0.0);
        // }
    }
}
