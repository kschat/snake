use std::{io::Write, thread::sleep, time::Duration};

use anyhow::{anyhow, Result};
use crossterm::event::{self, Event};

use crate::{
    renderer::{DrawInstruction, Renderer},
    timestep::Timestep,
};

pub enum GameLoopSignal {
    Run,
    Stop,
    Pause,
}

pub trait GameScene {
    fn draw<'a>(&'a mut self) -> Vec<DrawInstruction<'a>>;
    fn update(&mut self, elapsed: &Duration) -> Result<GameLoopSignal>;
    fn process_input(&mut self, event: &Event) -> Result<GameLoopSignal>;
}

pub struct GameLoop<W: Write> {
    renderer: Renderer<W>,
    ms_per_update: Duration,
    scenes: Vec<Box<dyn GameScene>>,
}

impl<W: Write> GameLoop<W> {
    pub fn new(renderer: Renderer<W>, frame_rate: u8) -> Self {
        Self {
            renderer,
            ms_per_update: Duration::from_millis((1_000.0 / (frame_rate as f32)) as u64),
            scenes: vec![],
        }
    }

    pub fn load_scene(&mut self, scene: Box<dyn GameScene>) {
        self.scenes.push(scene);
    }

    pub fn run(&mut self) -> Result<()> {
        let mut timestep = Timestep::new();
        let mut lag = Duration::from_millis(0);
        let mut state = GameLoopSignal::Run;
        let scene = self.scenes.get_mut(0).ok_or(anyhow!("No scene loaded"))?;

        self.renderer.start()?;

        'game_loop: loop {
            if event::poll(Duration::from_millis(0))? {
                state = scene.process_input(&event::read()?)?;
            }

            match state {
                GameLoopSignal::Stop => break,
                GameLoopSignal::Pause => continue,
                GameLoopSignal::Run => (),
            }

            lag += timestep.delta();
            while lag >= self.ms_per_update {
                lag -= self.ms_per_update;
                state = scene.update(&self.ms_per_update)?;
                match state {
                    GameLoopSignal::Stop => break 'game_loop,
                    GameLoopSignal::Pause => break,
                    GameLoopSignal::Run => (),
                }
            }

            self.renderer.draw(&scene.draw())?;

            let remaining_tick_time = self.ms_per_update - timestep.elapsed_time();
            if remaining_tick_time > Duration::from_millis(0) {
                sleep(remaining_tick_time);
            }
        }

        self.renderer.stop()
    }
}
