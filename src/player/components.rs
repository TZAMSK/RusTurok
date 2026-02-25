use bevy::{math::Vec3, prelude::Component};

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
    pub jump_height: f32,
    pub is_grounded: bool,
    pub velocity: f32,
    pub is_sprinting: bool,
    pub is_sliding: bool,
    pub is_crouching: bool,
    pub slide_distance: f32,
    pub slide_direction: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            speed: 13.0,
            jump_height: 7.0,
            is_grounded: true,
            velocity: 0.0,
            is_sprinting: false,
            is_sliding: false,
            is_crouching: false,
            slide_distance: 5.0,
            slide_direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
