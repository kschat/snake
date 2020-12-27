use std::{
    io::{stdout, Stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::poll,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

const TOP_LEFT: &'static str = "┌";
const TOP_RIGHT: &'static str = "┐";
const BOTTOM_RIGHT: &'static str = "┘";
const BOTTOM_LEFT: &'static str = "└";
const HORIZONTAL_LINE: &'static str = "─";
const VERTICAL_LINE: &'static str = "│";
const CELL: &'static str = "█";
const GAME_TICK: Duration = Duration::from_millis(33); // 1 update every 33 ms = 30 FPS

struct Snake {
    body: Vec<(u16, u16)>,
}

fn game_loop() -> Result<()> {
    let snake = Snake {
        body: vec![(2, 2), (2, 3), (2, 4)],
    };

    let mut stdout = stdout();
    let (width, height) = terminal::size()?;
    let width = width - 1;
    let height = height - 3;

    let mut previous = Instant::now();
    let mut delta: Duration = Duration::from_millis(0);

    loop {
        let now = Instant::now();
        delta += now.duration_since(previous);
        previous = now;

        poll(Duration::from_millis(10))?;

        while delta >= GAME_TICK {
            delta -= GAME_TICK;
            sleep(Duration::from_millis(10));
            // TODO update
        }

        draw(&mut stdout, width, height)?;

        let elapsed_time = Instant::now().duration_since(previous);
        if elapsed_time < GAME_TICK {
            sleep(GAME_TICK - elapsed_time);
        }
    }

    Ok(())
}

fn draw(stdout: &mut Stdout, width: u16, height: u16) -> Result<()> {
    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..height {
        for x in 0..width {
            if y == 0 && x == 0 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(TOP_LEFT.cyan()))?;
                continue;
            }

            if y == 0 && x == width - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(TOP_RIGHT.cyan()))?;
                continue;
            }

            if y == height - 1 && x == width - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(BOTTOM_RIGHT.cyan()))?;
                continue;
            }

            if y == height - 1 && x == 0 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(BOTTOM_LEFT.cyan()))?;
                continue;
            }

            if (y == 0 || y == height - 1) && x <= width - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(HORIZONTAL_LINE.cyan()))?;
                continue;
            }

            if (x == 0 || x == width - 1) && y <= height - 1 {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(VERTICAL_LINE.cyan()))?;
                continue;
            }
        }
    }

    stdout
        // -- first block
        .queue(cursor::MoveTo(2, 1))?
        .queue(style::PrintStyledContent(CELL.red()))?
        .queue(cursor::MoveTo(3, 1))?
        .queue(style::PrintStyledContent(CELL.red()))?
        // -- second block
        .queue(cursor::MoveTo(2, 2))?
        .queue(style::PrintStyledContent(CELL.red()))?
        .queue(cursor::MoveTo(3, 2))?
        .queue(style::PrintStyledContent(CELL.red()))?
        // -- third block
        .queue(cursor::MoveTo(4, 1))?
        .queue(style::PrintStyledContent(CELL.red()))?
        .queue(cursor::MoveTo(5, 1))?
        .queue(style::PrintStyledContent(CELL.red()))?
        .queue(cursor::MoveTo(0, height))?
        .flush()?;

    Ok(())
}

fn main() -> Result<()> {
    game_loop()
}
