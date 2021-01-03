use anyhow::{anyhow, Result};
use crossterm::event;
use std::{io::Write, thread::sleep, time::Duration};

use super::{renderer::Renderer, timestep::Timestep, traits::GameScene};

pub struct GameLoopConfig {
    pub frame_rate: u8,
    pub input_poll_rate: Duration,
}

pub enum GameLoopSignal {
    Run,
    Stop,
}

pub struct GameLoop<W: Write> {
    config: GameLoopConfig,
    renderer: Renderer<W>,
    ms_per_update: Duration,
    scenes: Vec<Box<dyn GameScene>>,
}

impl<W: Write> GameLoop<W> {
    pub fn new(renderer: Renderer<W>, config: GameLoopConfig) -> Self {
        let ms_per_update = Duration::from_millis((1_000.0 / (config.frame_rate as f32)) as u64);

        Self {
            config,
            renderer,
            ms_per_update,
            scenes: vec![],
        }
    }

    pub fn load_scene(&mut self, scene: Box<dyn GameScene>) {
        self.scenes.push(scene);
    }

    pub fn run(&mut self) -> Result<()> {
        let scene = self
            .scenes
            .get_mut(0)
            .ok_or_else(|| anyhow!("No scene loaded"))?;

        let mut timestep = Timestep::new();
        let mut lag = Duration::from_millis(0);
        let mut state = GameLoopSignal::Run;

        self.renderer.start()?;

        'game_loop: loop {
            if event::poll(self.config.input_poll_rate)? {
                state = scene.process_input(&event::read()?)?;
            }

            match state {
                GameLoopSignal::Stop => break,
                GameLoopSignal::Run => (),
            }

            lag += timestep.delta();
            while lag >= self.ms_per_update {
                lag -= self.ms_per_update;
                state = scene.update(&self.ms_per_update)?;
                match state {
                    GameLoopSignal::Stop => break 'game_loop,
                    GameLoopSignal::Run => (),
                }
            }

            self.renderer.draw(&scene.draw())?;

            let remaining_tick_time = self
                .ms_per_update
                .checked_sub(timestep.elapsed_time())
                .unwrap_or_default();

            if remaining_tick_time > Duration::from_millis(0) {
                sleep(remaining_tick_time);
            }
        }

        self.renderer.stop()
    }
}
