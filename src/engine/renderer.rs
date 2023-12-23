use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;

use anyhow::{Context, Result};
use crossterm::{
    cursor,
    style::{self, Color, Print},
    terminal, QueueableCommand,
};

use super::point::Point;

const PIXEL: &str = "█";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Pixel {
    pub content: String,
    pub fg: Color,
    pub bg: Color,
}

impl Pixel {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.into(),
            ..Self::default()
        }
    }

    pub fn with_fg(self, fg: Color) -> Self {
        Self { fg, ..self }
    }

    pub fn with_bg(self, bg: Color) -> Self {
        Self { bg, ..self }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            content: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

#[derive(Debug)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    rows: usize,
    columns: usize,
    pixels: HashMap<Point, Pixel>,
    previous: HashMap<Point, Pixel>,
}

impl FrameBuffer {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            pixels: HashMap::new(),
            previous: HashMap::new(),
        }
    }

    /// Iterates over the current buffer updating its state so it clears the
    /// the screen.
    pub fn clear(&mut self) {
        self.previous = self.pixels.clone();
        self.pixels.retain(|_, pixel| {
            let clear = Default::default();
            // If the previous pixel was cleared there's no reason to write
            // it again. Remove it from the buffer.
            if *pixel == clear {
                return false;
            }

            // If the pixel isn't a clear, then we need to clear it during the
            // next draw call.
            *pixel = clear;
            true
        });
    }

    pub fn frame_changed(&self) -> bool {
        self.previous != self.pixels
    }

    pub fn set_at(&mut self, position: Point, pixel: Pixel) {
        if position.x < self.columns && position.y < self.rows {
            self.pixels.insert(position, pixel);
        }
    }
}

impl<'a> IntoIterator for &'a FrameBuffer {
    type Item = <&'a HashMap<Point, Pixel> as IntoIterator>::Item;
    type IntoIter = <&'a HashMap<Point, Pixel> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.iter()
    }
}

#[derive(Debug)]
pub enum DrawInstruction<'a> {
    Square {
        position: Point,
        size: usize,
        style: Style,
    },
    Text {
        position: Point,
        content: &'a str,
        style: Style,
    },
}

impl<'a> DrawInstruction<'a> {
    pub fn apply(&self, buffer: &mut FrameBuffer) {
        match self {
            DrawInstruction::Square {
                position,
                size,
                style,
            } => {
                let height = *size;
                let width = 2 * height;

                for row in 0..height {
                    for column in 0..width {
                        let position = position + Point::new(column, row);
                        buffer.set_at(
                            position,
                            Pixel::new(PIXEL).with_fg(style.fg).with_bg(style.bg),
                        );
                    }
                }
            }

            DrawInstruction::Text {
                position,
                content,
                style,
            } => {
                content.chars().enumerate().for_each(|(i, c)| {
                    let position = position + Point::new(i, 0);
                    buffer.set_at(
                        position,
                        Pixel::new(&c.to_string())
                            .with_fg(style.fg)
                            .with_bg(style.bg),
                    );
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct Renderer<W: Write> {
    writer: W,
    buffer: FrameBuffer,
    running: bool,
}

impl<W: Write> Renderer<W> {
    pub fn new(writer: W, rows: usize, columns: usize) -> Self {
        let buffer = FrameBuffer::new(rows, columns);

        Self {
            writer,
            buffer,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.running {
            return Ok(());
        }

        self.running = true;
        terminal::enable_raw_mode()?;
        self.writer
            .queue(terminal::EnterAlternateScreen)?
            .queue(cursor::Hide)?
            .flush()
            .with_context(|| "Failed to prepare terminal for game")
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.running {
            return Ok(());
        }

        self.running = false;
        terminal::disable_raw_mode()?;
        self.writer
            .queue(terminal::LeaveAlternateScreen)?
            .queue(cursor::Show)?
            .flush()
            .with_context(|| "Failed to restore terminal to original state")
    }

    pub fn draw(&mut self, draw_instructions: &[DrawInstruction]) -> Result<()> {
        self.buffer.clear();

        for instruction in draw_instructions {
            instruction.apply(&mut self.buffer);
        }

        if !self.buffer.frame_changed() {
            return Ok(());
        }

        let mut previous_fg = Color::Reset;
        let mut previous_bg = Color::Reset;

        for (position, pixel) in &self.buffer {
            self.writer
                .queue(cursor::MoveTo(position.x as u16, position.y as u16))?;

            if pixel.fg != previous_fg {
                self.writer.queue(style::SetForegroundColor(pixel.fg))?;
                previous_fg = pixel.fg;
            }

            if pixel.bg != previous_bg {
                self.writer.queue(style::SetBackgroundColor(pixel.bg))?;
                previous_bg = pixel.bg;
            }

            self.writer.queue(Print(&pixel.content))?;
        }

        self.writer.flush()?;

        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        self.stop().unwrap();
    }
}
