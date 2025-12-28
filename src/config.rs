use crate::CommandOptions;

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub snake: SnakeConfig,
    pub rows: usize,
    pub columns: usize,
    pub show_frame_rate: bool,
    pub show_border: bool,
    pub frame_rate: u8,
}

impl GameConfig {
    pub fn new(command_options: CommandOptions, (columns, rows): (u16, u16)) -> Self {
        Self {
            snake: SnakeConfig {
                grow_rate: command_options.grow_rate,
                speed: command_options.speed,
            },
            columns: columns as usize,
            rows: rows as usize,
            show_frame_rate: command_options.show_frame_rate,
            show_border: command_options.show_border,
            frame_rate: command_options.frame_rate,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnakeConfig {
    pub grow_rate: usize,
    pub speed: f32,
}
