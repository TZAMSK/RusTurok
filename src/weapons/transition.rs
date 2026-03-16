use bevy::prelude::*;

use crate::{
    camera::components::FirstLayerCamera,
    weapons::components::{
        ads::ADS,
        animation::{ease_out_cubic, GunAnimation, WeaponAnimationState},
    },
};

pub fn apply_transition_animation(
    mut weapon_query: Query<(
        &mut WeaponAnimationState,
        &mut Transform,
        &GunAnimation,
        &ADS,
    )>,
    camera_query: Query<&Transform, (With<Camera>, With<FirstLayerCamera>, Without<GunAnimation>)>,
    time: Res<Time>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for (mut weap_state, mut transform, gun_animation, ads) in weapon_query.iter_mut() {
        if weap_state.animation_progress >= 1.0 {
            continue;
        }

        weap_state.animation_progress += weap_state.animation_transition_speed * time.delta_secs();
        weap_state.animation_progress = weap_state.animation_progress.clamp(0.0, 1.0);

        let t = ease_out_cubic(weap_state.animation_progress);

        let start_rotation = Quat::from_euler(
            EulerRot::YXZ,
            weap_state.previous_coords.1.y,
            weap_state.previous_coords.1.x,
            weap_state.previous_coords.1.z,
        );

        let effective_target_rotation = if ads.is_ads && ads.ads_progress > 0.0 {
            let ads_t = ads.ads_progress;
            let stance_rotation = Quat::from_euler(
                EulerRot::YXZ,
                weap_state.rotation.y,
                weap_state.rotation.x,
                weap_state.rotation.z,
            );
            stance_rotation.slerp(Quat::IDENTITY, ads_t)
        } else {
            Quat::from_euler(
                EulerRot::YXZ,
                weap_state.rotation.y,
                weap_state.rotation.x,
                weap_state.rotation.z,
            )
        };

        transform.rotation =
            camera_transform.rotation * start_rotation.slerp(effective_target_rotation, t);
    }
}
