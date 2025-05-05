mod bullets;
pub mod components;
pub mod systems;
mod tests;

use bevy::prelude::*;

use bullets::despawn_timed_entities;
use systems::{bullet_movement, spawn_bullets};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullets)
            .add_systems(Update, bullet_movement);
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_timed_entities);
    }
}
