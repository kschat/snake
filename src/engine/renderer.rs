use std::{io::Write, iter::repeat_with};

use anyhow::{Context, Result};
use crossterm::{
    cursor,
    style::{self, Color},
    terminal, QueueableCommand,
};

use super::point::{AbsPoint, Point};

const PIXEL: &str = "█";

#[derive(Debug)]
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
pub struct ScreenBuffer {
    rows: usize,
    columns: usize,
    pixels: Vec<Pixel>,
}

impl ScreenBuffer {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            pixels: repeat_with(Pixel::default).take(rows * columns).collect(),
        }
    }

    #[inline(always)]
    fn calculate_index(columns: usize, rows: usize, position: &AbsPoint) -> usize {
        if position.x >= columns {
            panic!("Tried to index out of buffer bounds: x too large");
        }

        if position.y >= rows {
            panic!("Tried to index out of buffer bounds: y too large");
        }

        position.y * columns + position.x
    }

    pub fn empty(&mut self) {
        self.pixels = self.pixels.iter().map(|_| Pixel::default()).collect();
    }

    pub fn get_at(&self, position: &AbsPoint) -> Option<&Pixel> {
        self.pixels
            .get(Self::calculate_index(self.columns, self.rows, position))
    }

    pub fn get_mut_at(&mut self, position: &AbsPoint) -> Option<&mut Pixel> {
        self.pixels
            .get_mut(Self::calculate_index(self.columns, self.rows, position))
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
    pub fn apply(&self, buffer: &mut ScreenBuffer) {
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
                        let position = AbsPoint::from(position) + Point::new(column, row);
                        *buffer.get_mut_at(&position).unwrap() =
                            Pixel::new(PIXEL).with_fg(style.fg).with_bg(style.bg);
                    }
                }
            }

            DrawInstruction::Text {
                position,
                content,
                style,
            } => {
                content.chars().enumerate().for_each(|(i, c)| {
                    let position = AbsPoint::from(position) + Point::new(i, 0);
                    *buffer.get_mut_at(&position).unwrap() = Pixel::new(&c.to_string())
                        .with_fg(style.fg)
                        .with_bg(style.bg);
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct Renderer<W: Write> {
    rows: usize,
    columns: usize,
    writer: W,
    buffer: ScreenBuffer,
    running: bool,
}

impl<W: Write> Renderer<W> {
    pub fn new(writer: W, rows: usize, columns: usize) -> Self {
        let buffer = ScreenBuffer::new(rows, columns);

        Self {
            rows,
            columns,
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
        self.buffer.empty();

        for instruction in draw_instructions {
            instruction.apply(&mut self.buffer);
        }

        for y in 0..self.rows {
            for x in 0..self.columns {
                let pixel = self.buffer.get_at(&Point::new(x, y)).unwrap();

                self.writer
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::PrintStyledContent(
                        style::style(pixel.content.clone())
                            .with(pixel.fg)
                            .on(pixel.bg),
                    ))?;
            }
        }

        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        self.stop().unwrap();
    }
}
