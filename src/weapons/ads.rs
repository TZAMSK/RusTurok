use crate::camera::components::{CameraSensitivity, FirstLayerCamera, WeaponLayerCamera};
use crate::camera::systems::{FIRST_LAYER_ADS_FOV, FIRST_LAYER_HIP_FOV};
use crate::player::components::Player;
use crate::weapons::components::ads::ADS;
use crate::weapons::components::animation::{
    GunAnimation, WeaponAnimationStance, WeaponAnimationState,
};
use crate::weapons::components::weapon::Weapon;
use crate::weapons::ressources::input::WeaponInput;
use bevy::prelude::*;

pub fn update_ads(
    mut weapon_input: ResMut<WeaponInput>,
    mut weapon_query: Query<
        (
            &mut ADS,
            &mut GunAnimation,
            &Weapon,
            &mut WeaponAnimationState,
            &Transform,
        ),
        With<Weapon>,
    >,
    mut first_layer_projection: Single<
        &mut Projection,
        (With<FirstLayerCamera>, Without<WeaponLayerCamera>),
    >,
    mut player_query: Query<&mut Player>,
    mut sens_query: Query<&mut CameraSensitivity>,
    time: Res<Time>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };
    let Ok(mut sens) = sens_query.single_mut() else {
        return;
    };

    if weapon_input.should_cancel_sprint {
        player.movement.is_sprinting = false;
    }

    if player.movement.is_sprinting && weapon_input.ads_pressed {
        weapon_input.ads_blocked = true;
    }

    let effective_ads = weapon_input.ads_pressed && !weapon_input.ads_blocked;

    let mut ads_progress: Option<f32> = None;

    for (mut ads, mut gun_animation, _weapon, mut weapon_animation_state, transform) in
        weapon_query.iter_mut()
    {
        if ads_progress.is_none() {
            ads_progress = Some(ads.ads_progress);
        }

        let was_ads = ads.is_ads;
        ads.is_ads = effective_ads;

        if effective_ads && !was_ads {
            player.movement.speed = player.movement.base_speed * 0.75;
            sens.current = sens.base * 0.8;
        } else if !effective_ads && was_ads {
            player.movement.speed = player.movement.base_speed;
            sens.current = sens.base;
        }

        let target_progress = if ads.is_ads { 1.0 } else { 0.0 };
        ads.ads_progress +=
            (target_progress - ads.ads_progress) * ads.ads_speed * time.delta_secs();
        ads.ads_progress = ads.ads_progress.clamp(0.0, 1.0);

        let target_position = ads.hip_position.lerp(ads.ads_position, ads.ads_progress);
        gun_animation.wobble.base_offset = target_position;

        let ads_factor = 1.0 - ads.ads_progress * 0.7;
        gun_animation.bob.bob_intensity = 0.01 * ads_factor;
        gun_animation.wobble.intensity = 0.02 * ads_factor;

        let current_translation = transform.translation;
        let current_rotation = transform.rotation.to_euler(EulerRot::YXZ);
        let current_rotation_vec =
            Vec3::new(current_rotation.1, current_rotation.0, current_rotation.2);

        if weapon_animation_state.stance == WeaponAnimationStance::AimingDownSight {
            weapon_animation_state.change_state_by_stance(
                WeaponAnimationStance::Grounded,
                current_translation,
                current_rotation_vec,
            );
        }
    }

    let Projection::Perspective(perspective) = first_layer_projection.as_mut() else {
        return;
    };
    if let Some(progress) = ads_progress {
        perspective.fov = FIRST_LAYER_HIP_FOV
            .lerp(FIRST_LAYER_ADS_FOV, progress)
            .to_radians();
    }
}
