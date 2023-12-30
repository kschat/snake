use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode},
    style::Color,
};
use std::time::Duration;

use crate::{
    engine::{
        game_loop::GameLoopSignal,
        point::Point,
        renderer::{DrawInstruction, Style},
        timestep::Timestep,
        traits::{Entity, GameScene},
    },
    entities::text::Text,
    snake_scene::SnakeScene,
    PlayerInput, SnakeConfig,
};

const TITLE: &str = "
███████╗███╗   ██╗ █████╗ ██╗  ██╗███████╗
██╔════╝████╗  ██║██╔══██╗██║ ██╔╝██╔════╝
███████╗██╔██╗ ██║███████║█████╔╝ █████╗
╚════██║██║╚██╗██║██╔══██║██╔═██╗ ██╔══╝
███████║██║ ╚████║██║  ██║██║  ██╗███████╗
╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝
";

const STATIC_SNAKE: &str = "
██████████████████████████████████████
██                                  ██
██
██
██
██
██
████████████████
              ██
      ██████████
";

const STATIC_FOOD: &str = "⬤";

#[derive(Debug)]
pub struct TitleScene {
    title_text: Text,
    static_snake: Text,
    static_food: Text,
    options: Vec<Text>,
    selected_index: usize,
}

impl TitleScene {
    pub fn new(config: SnakeConfig) -> Self {
        let origin = Point::new(0, 0);
        let diagonal = Point::new(config.columns - origin.x, config.rows - origin.y);
        let center = Self::get_center_position(origin, diagonal);

        let title_text = Text::default()
            .with_value(TITLE.into())
            .center(center - Point::new(0usize, 15))
            .with_fg(Color::Yellow)
            .show();

        let static_snake = Text::default()
            .with_value(STATIC_SNAKE.into())
            .center(center - Point::new(0usize, 8))
            .with_fg(Color::Green)
            .show();

        let static_food = Text::default()
            .with_value(STATIC_FOOD.into())
            .center(Point::new(center.x + 18, center.y - 4))
            .with_fg(Color::Red)
            .show();

        let options = vec![
            Text::default()
                .with_value("    NEW GAME    ".into())
                .center(center - Point::new(0usize, 5))
                .with_fg(Color::Black)
                .with_bg(Color::Yellow)
                .show(),
            Text::default()
                .with_value("    SETTINGS    ".into())
                .center(center - Point::new(0usize, 4))
                .with_fg(Color::Yellow)
                .show(),
            Text::default()
                .with_value("      QUIT      ".into())
                .center(center - Point::new(0usize, 3))
                .with_fg(Color::Yellow)
                .show(),
        ];

        Self {
            title_text,
            static_snake,
            static_food,
            options,
            selected_index: 0,
        }
    }

    pub fn get_center_position(origin: Point, diagonal: Point) -> Point {
        Point::new((origin.x + diagonal.x) / 2, (origin.y + diagonal.y) / 2)
    }
}

impl GameScene for TitleScene {
    fn draw(&mut self, _timestep: &Timestep) -> Vec<DrawInstruction> {
        vec![
            self.options
                .iter()
                .flat_map(|option| option.draw())
                .collect::<Vec<_>>(),
            self.title_text.draw(),
            self.static_snake.draw(),
            self.static_food.draw(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn update(&mut self, _elapsed: &Duration) -> Result<GameLoopSignal> {
        for (i, option) in self.options.iter_mut().enumerate() {
            option.style = if self.selected_index == i {
                Style {
                    fg: Color::Black,
                    bg: Color::Yellow,
                }
            } else {
                Style {
                    fg: Color::Yellow,
                    bg: Color::Reset,
                }
            };
        }

        Ok(GameLoopSignal::Run)
    }

    fn process_input(&mut self, event: &Event) -> Result<GameLoopSignal> {
        let input = match event {
            Event::Key(e) => match e.code {
                KeyCode::Enter => PlayerInput::Select,
                KeyCode::Char('q') => PlayerInput::Quit,
                KeyCode::Char('a') | KeyCode::Left => PlayerInput::Left,
                KeyCode::Char('s') | KeyCode::Down => PlayerInput::Down,
                KeyCode::Char('d') | KeyCode::Right => PlayerInput::Right,
                KeyCode::Char('w') | KeyCode::Up => PlayerInput::Up,
                _ => PlayerInput::Noop,
            },
            _ => PlayerInput::Noop,
        };

        match input {
            PlayerInput::Quit => return Ok(GameLoopSignal::Stop),
            PlayerInput::Up => {
                self.selected_index = self
                    .selected_index
                    .wrapping_sub(1)
                    .clamp(0, self.options.len() - 1);
            }
            PlayerInput::Down => {
                self.selected_index = (self.selected_index + 1) % self.options.len();
            }
            PlayerInput::Select => match self.selected_index {
                0 => return Ok(GameLoopSignal::load_scene::<SnakeScene>()),
                2 => return Ok(GameLoopSignal::Stop),
                _ => (),
            },
            _ => (),
        }

        Ok(GameLoopSignal::Run)
    }
}
