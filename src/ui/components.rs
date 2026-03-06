use bevy::{ecs::entity::Entity, math::Vec2, prelude::Component};

#[derive(Component)]
pub struct XPBarFill;

#[derive(Component)]
pub struct KillsBarFill;

#[derive(Component)]
pub struct LevelIndicator;

#[derive(Component)]
pub struct WeaponLVLIndicator;

#[derive(Component)]
pub struct XPRequiredIndicator;

#[derive(Component)]
pub struct GivenXPIndicator;

#[derive(Component)]
pub struct BulletCount;
