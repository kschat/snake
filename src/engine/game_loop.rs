use anyhow::{anyhow, Result};
use crossterm::event;
use std::{any::TypeId, collections::HashMap, io::Write, thread::sleep, time::Duration};

use super::{renderer::Renderer, timestep::Timestep, traits::GameScene};

pub struct GameLoopConfig {
    pub frame_rate: u8,
    pub input_poll_rate: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum GameLoopSignal {
    Run,
    Stop,
    Load(TypeId),
}

impl GameLoopSignal {
    pub fn load_scene<T: GameScene>() -> Self {
        Self::Load(TypeId::of::<T>())
    }
}

pub struct GameLoop<W: Write> {
    config: GameLoopConfig,
    renderer: Renderer<W>,
    ms_per_update: Duration,
    scenes: HashMap<TypeId, Box<dyn GameScene>>,
}

impl<W: Write> GameLoop<W> {
    pub fn new(renderer: Renderer<W>, config: GameLoopConfig) -> Self {
        let ms_per_update = Duration::from_millis((1_000.0 / (config.frame_rate as f32)) as u64);

        Self {
            config,
            renderer,
            ms_per_update,
            scenes: HashMap::new(),
        }
    }

    pub fn register_scene<TScene: GameScene>(&mut self, scene: TScene) -> &mut Self {
        self.scenes.insert(TypeId::of::<TScene>(), Box::new(scene));
        self
    }

    pub fn run<TInitScene: GameScene>(&mut self) -> Result<()> {
        let mut timestep = Timestep::new();
        let mut lag = Duration::from_millis(0);
        let mut state = GameLoopSignal::Run;
        let mut next_scene = None;
        let mut scene = self
            .scenes
            .get_mut(&TypeId::of::<TInitScene>())
            .ok_or_else(|| anyhow!("No scene loaded"))?;

        self.renderer.start()?;

        'game_loop: loop {
            if let Some(scene_id) = next_scene {
                next_scene = None;
                scene = self
                    .scenes
                    .get_mut(&scene_id)
                    .ok_or_else(|| anyhow!("Scene with ID {scene_id:?} is not registered"))?;
            }

            if event::poll(self.config.input_poll_rate)? {
                state = scene.process_input(&event::read()?)?;
            }

            match state {
                GameLoopSignal::Stop => break,
                GameLoopSignal::Run => (),
                GameLoopSignal::Load(scene_id) => {
                    next_scene = Some(scene_id);
                }
            }

            lag += timestep.delta();
            while lag >= self.ms_per_update {
                lag -= self.ms_per_update;
                state = scene.update(&self.ms_per_update)?;
                match scene.update(&self.ms_per_update)? {
                    GameLoopSignal::Stop => break 'game_loop,
                    GameLoopSignal::Run => (),
                    GameLoopSignal::Load(scene_id) => {
                        next_scene = Some(scene_id);
                    }
                }
            }

            self.renderer.draw(&scene.draw(&timestep))?;

            let remaining_tick_time = self
                .ms_per_update
                .checked_sub(timestep.elapsed_time())
                .unwrap_or_default();

            if remaining_tick_time > Duration::from_millis(0) {
                sleep(remaining_tick_time);
            }

            timestep.track_frame();
        }

        self.renderer.stop()
    }
}
