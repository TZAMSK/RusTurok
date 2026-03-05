use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug, PartialEq)]
pub struct Recoil {
    pub pattern: Vec<Vec2>,
    pub current_bullet_index: u32,
    pub recoil_reset_time: f32,
    pub time_since_last_shot: f32,
}
