pub mod animation;
mod bullets;
pub mod components;
pub mod systems;
mod tests;
pub mod wobble;

use bevy::prelude::*;

use animation::update_gun_animation;
use bullets::despawn_timed_entities;
use systems::{bullet_movement, spawn_bullets};

pub use animation::GunAnimationState;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullets)
            .add_systems(Update, bullet_movement)
            .add_systems(Update, update_gun_animation);
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_timed_entities);
    }
}
