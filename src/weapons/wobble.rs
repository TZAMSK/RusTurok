use bevy::prelude::*;

#[derive(Component)]
pub struct GunWobble {
    pub base_offset: Vec3,
    pub current_offset: Vec3,
    pub intensity: f32,
    pub smoothness: f32,
    pub time: f32,
}

impl Default for GunWobble {
    fn default() -> Self {
        Self {
            base_offset: Vec3::new(0.0, 0.05, 0.7),
            current_offset: Vec3::ZERO,
            intensity: 0.02,
            smoothness: 0.1,
            time: 0.0,
        }
    }
}

#[derive(Component)]
pub struct GunBob {
    pub bob_intensity: f32,
    pub bob_speed: f32,
    pub phase: f32,
}

impl Default for GunBob {
    fn default() -> Self {
        Self {
            bob_intensity: 0.01,
            bob_speed: 10.0,
            phase: 0.0,
        }
    }
}
