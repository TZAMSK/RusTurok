use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub jump_height: f32,
    pub is_grounded: bool,
    pub velocity: f32,
    pub is_sprinting: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            speed: 13.0,
            jump_height: 7.0,
            is_grounded: true,
            velocity: 0.0,
            is_sprinting: false,
        }
    }
}
