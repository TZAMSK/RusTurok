use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Recoil {
    pub base_pattern: Vec<Vec2>,
    pub pattern: Vec<Vec2>,
    pub recoil_reset_time: f32,
    pub current_bullet_index: u32,
    pub time_since_last_shot: f32,
}
