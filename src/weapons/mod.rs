mod ads;
pub mod animation;
pub mod attachements;
pub mod bullets;
pub mod components;
pub mod data;
pub mod recoil;
pub mod reload;
pub mod ressources;
pub mod systems;
pub mod transition;

use bevy::prelude::*;

use crate::weapons::attachements::spawn_attachment_on_sockets;
use crate::weapons::recoil::apply_recoil;
use crate::weapons::ressources::input::handle_weapon_input;
use crate::weapons::systems::WeaponSpawnEvent;
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
        app.add_message::<WeaponSpawnEvent>()
            .add_message::<DamageMessage>()
            .init_resource::<ressources::input::WeaponInput>()
            .add_systems(
                Startup,
                (send_initial_weapon, spawn_weapon)
                    .chain()
                    .after(crate::camera::systems::spawn_camera),
            )
            .add_systems(PostUpdate, spawn_attachment_on_sockets)
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

fn send_initial_weapon(mut events: MessageWriter<WeaponSpawnEvent>) {
    events.write(WeaponSpawnEvent {
        weapon_id: "ak47".to_string(),
    });
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_timed_entities);
    }
}
