use anyhow::Result;
use crossterm::event;
use std::{any::TypeId, io::Write, thread::sleep, time::Duration};

use super::{
    renderer::Renderer, scene_manager::SceneManager, timestep::Timestep, traits::GameScene,
};

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
    scene_manager: SceneManager,
}

impl<W: Write> GameLoop<W> {
    pub fn new(renderer: Renderer<W>, config: GameLoopConfig) -> Self {
        let ms_per_update = Duration::from_millis((1_000.0 / (config.frame_rate as f32)) as u64);

        Self {
            config,
            renderer,
            ms_per_update,
            scene_manager: SceneManager::new(),
        }
    }

    pub fn register_scene<TScene: GameScene>(&mut self, scene: TScene) -> &mut Self {
        self.scene_manager.register(scene);
        self
    }

    pub fn run<TInitScene: GameScene>(&mut self) -> Result<()> {
        let mut frame_state = FrameState::new();
        let mut next_scene = None;
        let mut scene = self.scene_manager.load_mut::<TInitScene>()?;

        self.renderer.start()?;

        'game_loop: loop {
            if let Some(scene_id) = next_scene.take() {
                scene = self.scene_manager.load_mut_by_id(&scene_id)?;
            }

            if event::poll(self.config.input_poll_rate)? {
                frame_state.signal = scene.process_input(&event::read()?)?;
            }

            next_scene = match frame_state.signal {
                GameLoopSignal::Stop => break,
                GameLoopSignal::Run => next_scene,
                GameLoopSignal::Load(scene_id) => Some(scene_id),
            };

            frame_state.lag += frame_state.timestep.delta();
            while frame_state.lag >= self.ms_per_update {
                frame_state.lag -= self.ms_per_update;
                frame_state.signal = scene.update(&self.ms_per_update)?;
                next_scene = match frame_state.signal {
                    GameLoopSignal::Stop => break 'game_loop,
                    GameLoopSignal::Run => next_scene,
                    GameLoopSignal::Load(scene_id) => Some(scene_id),
                };
            }

            self.renderer.draw(&scene.draw(&frame_state.timestep))?;

            let remaining_tick_time = self
                .ms_per_update
                .checked_sub(frame_state.timestep.elapsed_time())
                .unwrap_or_default();

            if remaining_tick_time > Duration::from_millis(0) {
                sleep(Duration::from_millis(1));
            }

            frame_state.timestep.track_frame();
        }

        self.renderer.stop()
    }
}

struct FrameState {
    pub timestep: Timestep,
    pub lag: Duration,
    pub signal: GameLoopSignal,
}

impl FrameState {
    pub fn new() -> Self {
        Self {
            timestep: Timestep::new(),
            lag: Duration::from_millis(0),
            signal: GameLoopSignal::Run,
        }
    }
}
