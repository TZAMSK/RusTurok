mod ads;
pub mod animation;
pub mod bullets;
pub mod components;
pub mod recoil;
pub mod reload;
pub mod ressources;
pub mod systems;
pub mod transition;
pub mod wobble;

use bevy::prelude::*;

use crate::weapons::recoil::apply_recoil;
use crate::weapons::ressources::input::handle_weapon_input;
use ads::update_ads;
use animation::update_gun_animation;
use bullets::despawn_timed_entities;
use systems::{spawn_bullets, spawn_weapon};

use crate::combat::DamageMessage;
use crate::weapons::reload::reload_weapon;
use crate::weapons::transition::apply_transition_animation;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageMessage>()
            .init_resource::<ressources::input::WeaponInput>()
            .add_systems(
                Startup,
                spawn_weapon.after(crate::camera::systems::spawn_camera),
            )
            .add_systems(
                Update,
                (
                    handle_weapon_input,
                    update_ads,
                    spawn_bullets,
                    apply_recoil,
                    update_gun_animation,
                    apply_transition_animation,
                    reload_weapon,
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
