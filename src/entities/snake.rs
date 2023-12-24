use crossterm::style::Color;
use std::{iter::repeat_with, time::Duration};

use crate::{
    engine::{
        point::{Point, Vector},
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
    PlayerInput,
};

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    size: usize,
    velocity: Vector,
    speed: f32,
    movement_progress: f32,
}

impl Snake {
    pub fn new(head: Point, size: usize, speed: f32) -> Self {
        let body = repeat_with(|| head)
            .enumerate()
            .map(|(index, point)| point + Point::new((size - index) * 2, 0))
            .take(size)
            .collect();

        Self {
            body,
            size,
            velocity: Point::new(2, 0),
            speed,
            movement_progress: 0.0,
        }
    }

    pub fn head(&self) -> Point {
        self.body[0]
    }

    pub fn detect_head_collision(&self, point: Point) -> bool {
        self.head() == point
    }

    pub fn detect_self_collision(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .any(|part| self.detect_head_collision(*part))
    }

    pub fn detect_collision(&self, point: Point) -> bool {
        self.body.iter().any(|part| *part == point)
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
                    ..Default::default()
                },
            })
            .collect()
    }

    fn update(&mut self, elapsed: &Duration) {
        self.movement_progress += self.speed * elapsed.as_secs_f32();
        while self.movement_progress > 1.0 {
            self.movement_progress -= 1.0;

            let head = self.head();

            if self.size != self.body.len() {
                self.body.insert(0, head);
            } else {
                self.body.rotate_right(1);
            }

            self.body[0] = head + self.velocity;
        }
    }

    fn process_input(&mut self, input: &Self::Input) {
        // "squares" are 2x1 since fonts are taller than they are wide so we need to
        // move double the distance when going east or west
        self.velocity = match input {
            PlayerInput::Up if self.velocity.y == 0 => Vector::new(0, -1),
            PlayerInput::Down if self.velocity.y == 0 => Vector::new(0, 1),
            PlayerInput::Right if self.velocity.x == 0 => Vector::new(2, 0),
            PlayerInput::Left if self.velocity.x == 0 => Vector::new(-2, 0),
            _ => self.velocity,
        };
    }
}
