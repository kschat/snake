use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, io::Write, iter::repeat_with};

use anyhow::{Context, Result};
use crossterm::{
    cursor,
    style::{self, Color, Print},
    terminal, QueueableCommand,
};

use super::point::Point;

const PIXEL: &str = "█";

#[derive(Debug, Eq)]
pub struct Pixel {
    pub content: String,
    pub fg: Color,
    pub bg: Color,
    dirty: bool,
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

    pub fn set_dirty(self) -> Self {
        Self {
            dirty: true,
            ..self
        }
    }
}

impl PartialEq for Pixel {
    // don't compare the dirty flag for equality
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.fg == other.fg && self.bg == other.bg
    }
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content.hash(state);
        self.fg.hash(state);
        self.bg.hash(state);
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            content: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            dirty: false,
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
    hash: u64,
}

impl ScreenBuffer {
    pub fn new(rows: usize, columns: usize) -> Self {
        let pixels = repeat_with(Default::default)
            .take(rows * columns)
            .collect::<Vec<_>>();

        Self {
            rows,
            columns,
            hash: Self::calculate_hash(&pixels),
            pixels,
        }
    }

    #[inline(always)]
    fn calculate_hash(pixels: &[Pixel]) -> u64 {
        let mut hasher = DefaultHasher::new();
        pixels.hash(&mut hasher);
        hasher.finish()
    }

    #[inline(always)]
    fn calculate_index(columns: usize, rows: usize, position: &Point) -> usize {
        if position.x >= columns {
            panic!("Tried to index out of buffer bounds: x too large");
        }

        if position.y >= rows {
            panic!("Tried to index out of buffer bounds: y too large");
        }

        position.y * columns + position.x
    }

    pub fn update_hash(&mut self) -> u64 {
        self.hash = Self::calculate_hash(&self.pixels);
        self.hash
    }

    pub fn empty(&mut self) {
        self.pixels.iter_mut().for_each(|pixel| {
            let mut clear = Default::default();
            if *pixel != clear {
                clear.dirty = true;
            }
            *pixel = clear;
        });
    }

    pub fn get_at(&self, position: &Point) -> Option<&Pixel> {
        self.pixels
            .get(Self::calculate_index(self.columns, self.rows, position))
    }

    pub fn get_at_mut(&mut self, position: &Point) -> Option<&mut Pixel> {
        self.pixels
            .get_mut(Self::calculate_index(self.columns, self.rows, position))
    }

    pub fn set_at(&mut self, position: &Point, pixel: Pixel) {
        let current_pixel = self.get_at_mut(position).unwrap();
        if *current_pixel != pixel {
            *current_pixel = pixel.set_dirty();
        }
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
                        let position = position + Point::new(column, row);
                        buffer.set_at(
                            &position,
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
                        &position,
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

    pub fn draw_diff(&mut self, draw_instructions: &[DrawInstruction]) -> Result<()> {
        let hash = self.buffer.hash;
        self.buffer.empty();

        for instruction in draw_instructions {
            instruction.apply(&mut self.buffer);
        }

        if hash == self.buffer.update_hash() {
            eprintln!("Hash the same, not drawing");
            return Ok(());
        }

        let mut previous_fg = Color::Reset;
        let mut previous_bg = Color::Reset;
        let mut draw_counter = 0;

        for y in 0..self.rows {
            for x in 0..self.columns {
                let pixel = self.buffer.get_at(&Point::new(x, y)).unwrap();
                if !pixel.dirty {
                    continue;
                }

                draw_counter += 1;
                self.writer.queue(cursor::MoveTo(x as u16, y as u16))?;

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
        }

        self.writer.flush()?;
        eprintln!("Draw calls this step: {draw_counter}");

        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        self.stop().unwrap();
    }
}
