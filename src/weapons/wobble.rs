use bevy::prelude::*;

#[derive(Component)]
pub struct GunWobble {
    pub base_offset: Vec3,
    pub current_offset: Vec3,
    pub speed: f32,
    pub intensity: f32,
    pub smoothness: f32,
    pub last_velocity: f32,
    pub time: f32,
}

impl Default for GunWobble {
    fn default() -> Self {
        Self {
            base_offset: Vec3::new(0.0, 0.05, 0.7),
            current_offset: Vec3::ZERO,
            speed: 8.0,
            intensity: 0.02,
            smoothness: 0.1,
            last_velocity: 0.0,
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
