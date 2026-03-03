use bevy::{ecs::entity::Entity, math::Vec2, prelude::Component};

#[derive(Component)]
pub struct XPBarFill;

#[derive(Component)]
pub struct LevelIndicator;

#[derive(Component)]
pub struct XPRequiredIndicator;

#[derive(Component)]
pub struct GivenXPIndicator;

#[derive(Component)]
pub struct BulletCount;

#[derive(Component)]
pub struct DMGIndicator {
    pub enemy: Entity,
    pub animation_progress: f32,
    pub animation_complete: bool,
    pub animation_speed: f32,
    pub base_offset: Vec2,
    pub drift_right: bool,
}
