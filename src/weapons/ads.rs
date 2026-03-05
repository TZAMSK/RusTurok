use bevy::prelude::*;

use super::components::{GunAnimation, Weapon, ADS};
use crate::camera::components::FirstLayerCamera;
use crate::player::components::Player;
use crate::weapons::ressources::input::WeaponInput;

pub fn update_ads(
    weapon_input: Res<WeaponInput>,
    mut weapon_query: Query<(&mut ADS, &mut GunAnimation, &Weapon), With<Weapon>>,
    mut camera_query: Query<&mut Projection, (With<Camera>, With<FirstLayerCamera>)>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    if weapon_input.should_cancel_sprint {
        if let Ok(mut player) = player_query.single_mut() {
            player.movement.is_sprinting = false;
        }
    }

    let mut first_ads_data: Option<(f32, f32, f32)> = None;
    for (mut ads, mut gun_animation, _weapon) in weapon_query.iter_mut() {
        if first_ads_data.is_none() {
            first_ads_data = Some((ads.hip_fov, ads.ads_fov, ads.ads_progress));
        }

        ads.is_ads = weapon_input.ads_pressed;

        let target_progress = if ads.is_ads { 1.0 } else { 0.0 };
        ads.ads_progress +=
            (target_progress - ads.ads_progress) * ads.ads_speed * time.delta_secs();
        ads.ads_progress = ads.ads_progress.clamp(0.0, 1.0);

        let target_position = ads.hip_position.lerp(ads.ads_position, ads.ads_progress);
        gun_animation.wobble.base_offset = target_position;

        let ads_factor = 1.0 - ads.ads_progress * 0.7;
        gun_animation.bob.bob_intensity = 0.01 * ads_factor;
        gun_animation.wobble.intensity = 0.02 * ads_factor;
    }

    if let Ok(projection) = camera_query.single_mut() {
        if let Projection::Perspective(perspective) = projection.into_inner() {
            if let Some((hip_fov, ads_fov, ads_progress)) = first_ads_data {
                let target_fov = hip_fov.lerp(ads_fov, ads_progress);
                perspective.fov = target_fov.to_radians();
            }
        }
    }
}
