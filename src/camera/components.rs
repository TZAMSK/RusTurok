use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct CameraSensitivity {
    pub current: Vec2,
    pub base: Vec2,
}

#[derive(Debug, Component)]
pub struct FirstLayerCamera;

#[derive(Debug, Component)]
pub struct WeaponLayerCamera;

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self {
            current: Vec2::new(0.00043, 0.00043),
            base: Vec2::new(0.00043, 0.00043),
        }
    }
}
