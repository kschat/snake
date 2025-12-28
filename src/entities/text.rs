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
    pub position: Point,
    pub visible: bool,
    pub style: Style,
}

impl Text {
    pub fn with_value(mut self, value: String) -> Self {
        self.update_value(value);
        self
    }

    pub fn update_value(&mut self, value: String) {
        self.value = value;
        self.longest_width = self
            .value
            .split('\n')
            .map(|line| line.graphemes(true).count())
            .fold(usize::MIN, |a, b| a.max(b));
    }

    pub fn get_value(&self) -> &str {
        &self.value
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

    pub fn center<T: Into<Point>>(self, center_point: T) -> Self {
        let center_point = center_point.into();
        let position = center_point - Point::new(self.longest_width / 2, 0);

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
