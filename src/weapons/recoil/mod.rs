pub mod auto_rifle_patterns;
pub mod components;

use super::components::BulletTracer;
use crate::{
    camera::components::FirstLayerCamera,
    combat::DamageMessage,
    enemy::components::Enemy,
    player::components::Player,
    weapons::{components::Weapon, ressources::input::WeaponInput},
};
use bevy::prelude::*;

pub fn apply_recoil(
    weapon_input: Res<WeaponInput>,
    mut weapon_query: Query<(&mut Weapon, &GlobalTransform)>,
    camera_query: Query<&GlobalTransform, (With<Camera>, With<FirstLayerCamera>)>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let Ok((mut weapon, _weapon_transform)) = weapon_query.single_mut() else {
        return;
    };

    weapon.fire_cooldown = (weapon.fire_cooldown - time.delta_secs()).max(0.0);

    if weapon_input.shoot_pressed {
        let Ok(camera_transform) = camera_query.single() else {
            return;
        };

        weapon.unique_trait.current_magazine_bullets -= 1;
        weapon.fire_cooldown = weapon.unique_trait.stats.seconds_per_shot;
    }
}

pub fn apply_stability(pattern: &[Vec2], stability: f32) -> Vec<Vec2> {
    let calc_stability = 1.0 - (stability / 100.0);
    pattern
        .iter()
        .map(|coord| *coord * calc_stability)
        .collect()
}
