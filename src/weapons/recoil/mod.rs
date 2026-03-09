pub mod auto_rifle_patterns;
pub mod components;

use crate::{
    player::components::Player,
    weapons::{components::weapon::Weapon, ressources::input::WeaponInput},
};
use bevy::prelude::*;

pub fn apply_recoil(
    weapon_input: Res<WeaponInput>,
    mut weapon_query: Query<(&mut Weapon, &GlobalTransform)>,
    player_query: Single<&mut Transform, With<Player>>,
) {
    let Ok((mut weapon, _weapon_transform)) = weapon_query.single_mut() else {
        return;
    };

    let mut transform = player_query.into_inner();

    if weapon_input.shoot_pressed {
        if weapon.unique_trait.recoil.current_bullet_index < weapon.unique_trait.mag_size - 1 {
            weapon.unique_trait.recoil.current_bullet_index =
                weapon.unique_trait.mag_size - weapon.unique_trait.current_magazine_bullets - 1;

            let Some(bounce) = weapon
                .unique_trait
                .recoil
                .pattern
                .get(weapon.unique_trait.recoil.current_bullet_index as usize)
            else {
                return;
            };

            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let yaw = yaw + bounce.x / 180.0;
            let pitch = pitch + bounce.y / 180.0;

            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }
    }
}

pub fn apply_stability(pattern: &[Vec2], stability: f32) -> Vec<Vec2> {
    let calc_stability = (100.0 - stability) / 100.0 + 0.05;
    pattern
        .iter()
        .map(|coord| *coord * calc_stability)
        .collect()
}
