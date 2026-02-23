mod ads;
pub mod animation;
mod bullets;
pub mod components;
pub mod systems;
mod tests;
pub mod wobble;

use bevy::prelude::*;

use ads::{handle_ads_input, update_ads};
use animation::update_gun_animation;
use bullets::despawn_timed_entities;
use systems::{bullet_movement, spawn_bullets};

use crate::weapons::systems::bullet_hit_enemy;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ads::ADSInput>().add_systems(
            Update,
            (
                handle_ads_input,
                update_ads,
                spawn_bullets,
                bullet_movement,
                update_gun_animation,
                bullet_hit_enemy,
            )
                .chain(),
        );
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_timed_entities);
    }
}
