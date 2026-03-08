use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(pub Vec2);

#[derive(Debug, Component)]
pub struct FirstLayerCamera;

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.00043, 0.00043))
    }
}
