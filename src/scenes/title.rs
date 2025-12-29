use anyhow::{Result, anyhow};
use crossterm::{
    event::{Event, KeyCode},
    style::Color,
};
use std::{fmt::Display, time::Duration};

use crate::{
    GameConfig, PlayerInput,
    engine::{
        game_loop::GameLoopSignal,
        point::Point,
        renderer::{DrawInstruction, Style},
        timestep::Timestep,
        traits::{Entity, GameScene},
    },
    entities::text::Text,
};

use super::snake::SnakeScene;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum MenuOption {
    NewGame = 0,
    Settings,
    Exit,
}

impl MenuOption {
    pub fn perform_action(&self) -> GameLoopSignal {
        match self {
            Self::NewGame => GameLoopSignal::load_scene::<SnakeScene>(),
            Self::Exit => GameLoopSignal::Stop,
            _ => GameLoopSignal::Run,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::NewGame, Self::Settings, Self::Exit].iter().copied()
    }
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewGame => write!(f, "    NEW GAME    "),
            Self::Settings => write!(f, "    SETTINGS    "),
            Self::Exit => write!(f, "      EXIT      "),
        }
    }
}

impl TryFrom<usize> for MenuOption {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::NewGame,
            1 => Self::Settings,
            2 => Self::Exit,
            _ => return Err(anyhow!("Failed to convert {value} to MenuOption")),
        })
    }
}

#[derive(Debug)]
pub struct TitleScene {
    title_text: Text,
    static_snake: Text,
    static_food: Text,
    options: Vec<Text>,
    selected_index: usize,
}

impl TitleScene {
    pub fn new(config: GameConfig) -> Self {
        let origin = Point::new(0, 0);
        let diagonal = Point::new(config.columns - origin.x, config.rows - origin.y);
        let center = Self::get_center_position(origin, diagonal);

        let title_text = Text::default()
            .with_value(TITLE)
            .center(center - Point::new(0usize, 15))
            .with_fg(Color::Yellow)
            .show();

        let static_snake = Text::default()
            .with_value(STATIC_SNAKE)
            .center(center - Point::new(0usize, 8))
            .with_fg(Color::Green)
            .show();

        let static_food = Text::default()
            .with_value(STATIC_FOOD)
            .center(Point::new(center.x + 18, center.y - 4))
            .with_fg(Color::Red)
            .show();

        let options = MenuOption::iter()
            .enumerate()
            .map(|(i, option)| {
                let text = Text::default()
                    .with_value(option.to_string())
                    .center(center - Point::new(0usize, 5 - i))
                    .with_fg(Color::Yellow)
                    .show();

                if i == 0 {
                    return text.with_fg(Color::Black).with_bg(Color::Yellow);
                }

                text
            })
            .collect();

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

    pub fn select_previous_option(&mut self) {
        self.selected_index = self
            .selected_index
            .wrapping_sub(1)
            .clamp(0, self.options.len() - 1);
    }

    pub fn select_next_option(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.options.len();
    }
}

impl GameScene for TitleScene {
    fn draw(&mut self, _timestep: &Timestep) -> Vec<DrawInstruction<'_>> {
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
            PlayerInput::Up => self.select_previous_option(),
            PlayerInput::Down => self.select_next_option(),
            PlayerInput::Select => {
                return Ok(MenuOption::try_from(self.selected_index)?.perform_action());
            }
            _ => (),
        }

        Ok(GameLoopSignal::Run)
    }
}
