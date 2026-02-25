mod ads;
pub mod animation;
mod bullets;
pub mod components;
pub mod systems;
mod tests;
mod transition;
pub mod wobble;

use bevy::prelude::*;

use ads::{handle_ads_input, update_ads};
use animation::update_gun_animation;
use bullets::despawn_timed_entities;
use systems::spawn_bullets;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ads::ADSInput>().add_systems(
            Update,
            (
                handle_ads_input,
                update_ads,
                spawn_bullets,
                update_gun_animation,
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
