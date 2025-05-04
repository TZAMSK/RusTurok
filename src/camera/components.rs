use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

#[derive(Debug, Component)]
pub struct FirstLayerCamera;

#[derive(Debug, Component)]
pub struct SecondLayerCamera;

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.001, 0.001))
    }
}
