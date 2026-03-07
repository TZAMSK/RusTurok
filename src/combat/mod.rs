mod dmg;
mod hitmark;

use bevy::prelude::*;

use crate::combat::{
    dmg::{dmg_indicator_spawn, update_dmg_indicator},
    hitmark::{spawn_hitmark, update_size},
};

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32,
    pub shooter: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct HitDetection {
    pub hit: bool,
    pub killed: bool,
    pub despawn_time: f32,
    pub hit_time: f32,
}

pub fn handle_hit_detection(mut hit_detection: ResMut<HitDetection>, time: Res<Time>) {
    hit_detection.despawn_time = 0.1;

    if hit_detection.hit
        && time.elapsed_secs() > hit_detection.hit_time + hit_detection.despawn_time
    {
        hit_detection.hit = false;
        hit_detection.killed = false;
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HitDetection>()
            .add_systems(Update, dmg_indicator_spawn)
            .add_systems(Update, update_dmg_indicator)
            .add_systems(Update, spawn_hitmark)
            .add_systems(Update, handle_hit_detection)
            .add_systems(Update, update_size);
    }
}
