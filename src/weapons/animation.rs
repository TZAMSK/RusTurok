use crate::{
    camera::components::FirstLayerCamera,
    player::components::Player,
    weapons::components::{
        ads::ADS,
        animation::{GunAnimation, WeaponAnimationStance, WeaponAnimationState},
        weapon::Weapon,
    },
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GunAnimationState {
    pub last_player_position: Vec3,
    pub velocity: Vec3,
}

impl Default for GunAnimationState {
    fn default() -> Self {
        Self {
            last_player_position: Vec3::ZERO,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn update_gun_animation(
    mut animation_state: ResMut<GunAnimationState>,
    player_query: Query<(&Player, &Transform), With<Player>>,
    mut gun_query: Query<
        (
            &mut Transform,
            &mut GunAnimation,
            &mut Weapon,
            &ADS,
            &mut WeaponAnimationState,
        ),
        Without<Player>,
    >,
    camera_query: Query<
        &Transform,
        (
            With<Camera>,
            With<FirstLayerCamera>,
            Without<Player>,
            Without<GunAnimation>,
        ),
    >,
    time: Res<Time>,
) {
    let Ok((player, player_transform)) = player_query.single() else {
        return;
    };
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    update_velocity(&mut animation_state, player_transform, &time);

    let speed = animation_state.velocity.length();
    let movement_dir = animation_state.velocity.normalize_or_zero();

    for (mut gun_transform, mut animation, _weapon, ads, mut weapon_animation_state) in
        gun_query.iter_mut()
    {
        update_animation_timers(&mut animation, &time);

        let ads_factor = calculate_ads_factor(ads, 0.99);
        let target_offset = calculate_target_offset(&animation, speed, player, ads);

        if player.movement.is_grounded == true {
            smooth_offset(&mut animation, target_offset);
        }

        apply_idle_animation(&mut animation, speed, ads);
        apply_gun_transform(&mut gun_transform, &animation);
        apply_gun_rotation(
            &mut gun_transform,
            &mut weapon_animation_state,
            &camera_transform,
            &movement_dir,
            speed,
            ads,
            ads_factor,
            player.movement.is_grounded,
            player.movement.is_sliding,
            player.movement.is_sprinting,
        );
    }
}

fn update_velocity(
    state: &mut ResMut<GunAnimationState>,
    player_transform: &Transform,
    time: &Time,
) {
    let current_position = player_transform.translation;

    if state.last_player_position != Vec3::ZERO {
        state.velocity =
            ((current_position - state.last_player_position) / time.delta_secs()) * 0.5;
    }
    state.last_player_position = current_position;
}

fn update_animation_timers(animation: &mut GunAnimation, time: &Time) {
    animation.wobble.time += time.delta_secs();
    animation.bob.phase += time.delta_secs() * animation.bob.bob_speed;
}

fn calculate_ads_factor(ads: &ADS, effect: f32) -> f32 {
    if ads.is_ads {
        1.0 - (ads.ads_progress * effect)
    } else {
        1.0
    }
}

fn calculate_target_offset(
    animation: &GunAnimation,
    speed: f32,
    player: &Player,
    ads: &ADS,
) -> Vec3 {
    if speed > 0.1 {
        calculate_movement_offset(animation, speed, player, ads)
    } else {
        Vec3::ZERO
    }
}

fn calculate_movement_offset(
    animation: &GunAnimation,
    speed: f32,
    player: &Player,
    ads: &ADS,
) -> Vec3 {
    let phase = animation.bob.phase;
    let up;
    let sideways;

    if player.movement.is_sprinting {
        up = phase.sin().abs() * animation.bob.bob_intensity * speed;
        sideways = phase.sin() * animation.wobble.intensity * speed;
    } else if ads.is_ads {
        let ads_factor = 0.3;
        up = phase.sin().abs() * animation.bob.bob_intensity * speed * ads_factor * 0.5;
        sideways = phase.sin() * animation.wobble.intensity * speed * ads_factor * 0.3;
    } else {
        let movement_factor = 0.1;
        up = phase.sin().abs() * animation.bob.bob_intensity * speed * movement_factor;
        sideways = phase.sin() * animation.wobble.intensity * speed * movement_factor;
    }

    Vec3::new(sideways, up, 0.0)
}

fn smooth_offset(animation: &mut GunAnimation, target_offset: Vec3) {
    animation.wobble.current_offset = animation
        .wobble
        .current_offset
        .lerp(target_offset, animation.wobble.smoothness);
}

fn apply_idle_animation(animation: &mut GunAnimation, speed: f32, ads: &ADS) {
    if speed < 0.1 || ads.is_ads {
        let idle_offset = calculate_idle_offset(animation);
        animation.wobble.current_offset = animation.wobble.current_offset.lerp(idle_offset, 0.15);
    }
}

fn calculate_idle_offset(animation: &GunAnimation) -> Vec3 {
    Vec3::new(
        (animation.wobble.time * 0.5).sin() * 0.005,
        (animation.wobble.time * 0.8).sin() * 0.003,
        0.0,
    )
}

fn apply_gun_transform(gun_transform: &mut Mut<Transform>, animation: &GunAnimation) {
    let final_offset = animation.wobble.base_offset + animation.wobble.current_offset;
    gun_transform.translation = final_offset;
}

fn apply_gun_rotation(
    gun_transform: &mut Mut<Transform>,
    weapon_animation_state: &mut WeaponAnimationState,
    camera_transform: &Transform,
    movement_dir: &Vec3,
    speed: f32,
    ads: &ADS,
    ads_factor: f32,
    is_grounded: bool,
    is_sliding: bool,
    is_sprinting: bool,
) {
    let in_transition = weapon_animation_state.animation_progress < 1.0;

    if weapon_animation_state.stance == WeaponAnimationStance::AimingDownSight || in_transition {
        return;
    }

    if speed > 0.1 && !ads.is_ads {
        if is_sprinting {
            let current_translation = gun_transform.translation;
            let current_rotation = gun_transform.rotation.to_euler(EulerRot::YXZ);
            let current_rotation_vec =
                Vec3::new(current_rotation.1, current_rotation.0, current_rotation.2);
            weapon_animation_state.change_state_by_stance(
                WeaponAnimationStance::Sprinting,
                current_translation,
                current_rotation_vec,
            );
        } else if is_sliding {
            let current_translation = gun_transform.translation;
            let current_rotation = gun_transform.rotation.to_euler(EulerRot::YXZ);
            let current_rotation_vec =
                Vec3::new(current_rotation.1, current_rotation.0, current_rotation.2);
            weapon_animation_state.change_state_by_stance(
                WeaponAnimationStance::Sliding,
                current_translation,
                current_rotation_vec,
            );
        }

        if !is_sliding && !is_sprinting {
            let roll = Quat::from_rotation_z(movement_dir.x * 0.1 * speed.min(1.0));
            if !is_grounded {
                let jump_tilt_factor = if ads.is_ads { ads_factor * 0.3 } else { 1.0 };
                let jump_pitch = Quat::from_rotation_x(-0.2 * jump_tilt_factor);
                gun_transform.translation.y -= 0.02 * jump_tilt_factor;
                gun_transform.rotation = camera_transform.rotation * roll * jump_pitch;
            } else {
                gun_transform.rotation = camera_transform.rotation * roll;
            }
        }
    } else {
        if !matches!(
            weapon_animation_state.stance,
            WeaponAnimationStance::Grounded
        ) {
            let current_translation = gun_transform.translation;
            let current_rotation = gun_transform.rotation.to_euler(EulerRot::YXZ);
            let current_rotation_vec =
                Vec3::new(current_rotation.1, current_rotation.0, current_rotation.2);
            weapon_animation_state.change_state_by_stance(
                WeaponAnimationStance::Grounded,
                current_translation,
                current_rotation_vec,
            );
        }
        gun_transform.rotation = camera_transform.rotation;
    }
}
