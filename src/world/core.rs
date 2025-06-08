use std::num::NonZeroUsize;

use bevy::{gltf::Gltf, prelude::*};

use crate::core::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Sun;

#[derive(Resource)]
pub struct MainScene {
    pub(super) levels: [Handle<Gltf>; LEVEL_COUNT],
    pub(super) is_spawned: bool,
    pub(super) skyboxes: [Handle<Image>; LEVEL_COUNT],
}

impl MainScene {
    pub(super) fn level(&self, level: NonZeroUsize) -> &Handle<Gltf> {
        &self.levels[level.get() - 1]
    }

    pub(super) fn skybox(&self, level: NonZeroUsize) -> &Handle<Image> {
        &self.skyboxes[level.get() - 1]
    }
}

#[derive(Resource)]
pub struct CurrentLevel(pub NonZeroUsize);

impl CurrentLevel {
    pub fn get(&self) -> NonZeroUsize {
        self.0
    }
}
