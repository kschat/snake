use anyhow::{anyhow, Result};
use std::{any::TypeId, collections::HashMap};

use super::traits::GameScene;

#[derive(Default)]
pub struct SceneManager {
    scenes: HashMap<TypeId, Box<dyn GameScene>>,
}

impl SceneManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register<TScene: GameScene>(&mut self, scene: TScene) -> &mut Self {
        self.scenes.insert(TypeId::of::<TScene>(), Box::new(scene));
        self
    }

    pub fn load_mut<TScene: GameScene>(&mut self) -> Result<&mut Box<dyn GameScene + 'static>> {
        self.load_mut_by_id(&TypeId::of::<TScene>())
    }

    pub fn load_mut_by_id(&mut self, id: &TypeId) -> Result<&mut Box<dyn GameScene + 'static>> {
        self.scenes
            .get_mut(id)
            .ok_or_else(|| anyhow!("Scene with ID {id:?} is not registered"))
    }
}
