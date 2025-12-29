use crossterm::style::Color;
use std::{iter::repeat_with, time::Duration};

use crate::{
    PlayerInput, SnakeStyle,
    config::SnakeConfig,
    engine::{
        point::{Point, Vector},
        renderer::{DrawInstruction, Style},
        traits::Entity,
    },
};

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    size: usize,
    velocity: Vector,
    speed: f32,
    movement_progress: f32,
    color: Color,
    color_time: Duration,
    config: SnakeConfig,
}

impl Snake {
    pub fn new<T: Into<Point>>(head: T, config: &SnakeConfig) -> Self {
        let head: Point = head.into();
        let body = repeat_with(|| head)
            .enumerate()
            .map(|(index, point)| point + Point::new((config.size - index) * 2, 0))
            .take(config.size)
            .collect();

        Self {
            body,
            size: config.size,
            speed: config.speed,
            velocity: Vector::new(2, 0),
            movement_progress: 0.0,
            color: config.style.initial_color(),
            color_time: Duration::from_secs(0),
            config: config.clone(),
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
        self.body.contains(&point)
    }

    pub fn grow(&mut self, amount: usize) {
        self.size += amount;
    }
}

impl Entity for Snake {
    type Input = PlayerInput;

    fn draw(&self) -> Vec<DrawInstruction<'_>> {
        self.body
            .iter()
            .map(|&position| DrawInstruction::Text {
                position,
                content: "██",
                style: Style {
                    fg: self.color,
                    ..Default::default()
                },
            })
            .collect()
    }

    fn update(&mut self, elapsed: &Duration) {
        self.color_time += *elapsed;
        self.movement_progress += self.speed * elapsed.as_secs_f32();

        if self.config.style == SnakeStyle::Flash {
            self.color = match (self.color, self.color_time > Duration::from_secs(1)) {
                (Color::Red, true) => Color::Green,
                (Color::Green, true) => Color::Yellow,
                (Color::Yellow, true) => Color::Blue,
                (Color::Blue, true) => Color::Red,
                _ => self.color,
            };
        }

        if self.color_time > Duration::from_secs(1) {
            self.color_time = Duration::from_secs(0);
        }

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
