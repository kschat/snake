#![allow(dead_code)]

use crossterm::style::Color;
use unicode_segmentation::UnicodeSegmentation;

use crate::engine::{
    point::Point,
    renderer::{DrawInstruction, Style},
    traits::Entity,
};

#[derive(Debug, Default)]
pub struct Text {
    value: String,
    longest_width: usize,
    center_point: Option<Point>,
    pub position: Point,
    pub visible: bool,
    pub style: Style,
}

impl Text {
    pub fn with_value<T: Into<String>>(mut self, value: T) -> Self {
        self.update_value(value.into());
        self
    }

    pub fn update_value<T: Into<String>>(&mut self, value: T) {
        self.value = value.into();
        self.longest_width = self
            .value
            .split('\n')
            .map(|line| line.graphemes(true).count())
            .fold(usize::MIN, |a, b| a.max(b));

        if let Some(center_point) = self.center_point {
            self.position = self.calcuate_center(center_point);
        }
    }

    pub fn at_position<T: Into<Point>>(mut self, position: T) -> Self {
        self.position = position.into();
        self
    }

    pub fn set_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn show(self) -> Self {
        self.set_visibility(true)
    }

    pub fn hide(self) -> Self {
        self.set_visibility(false)
    }

    pub fn toggle(self) -> Self {
        let visible = !self.visible;
        self.set_visibility(visible)
    }

    pub fn center<T: Into<Point>>(mut self, center_point: T) -> Self {
        let center_point = center_point.into();
        let position = self.calcuate_center(center_point);

        self.center_point = Some(center_point);
        self.at_position(position)
    }

    pub fn with_fg(mut self, fg: Color) -> Self {
        self.style.fg = fg;
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.style.bg = bg;
        self
    }

    fn calcuate_center(&self, center_point: Point) -> Point {
        center_point - Point::new(self.longest_width / 2, 0)
    }
}

impl Entity for Text {
    type Input = ();

    fn draw(&self) -> Vec<DrawInstruction<'_>> {
        if !self.visible {
            return vec![];
        }

        vec![DrawInstruction::Text {
            content: &self.value,
            position: self.position,
            style: self.style,
        }]
    }
}
