use crate::{camera::components::FirstLayerCamera, player::components::Player};
use bevy::prelude::*;

use super::components::{GunAnimation, Weapon, ADS};

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
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<(&mut Transform, &mut GunAnimation, &mut Weapon, &ADS), Without<Player>>,
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
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    update_velocity(&mut animation_state, player_transform, &time);
    let speed = animation_state.velocity.length();
    let movement_dir = animation_state.velocity.normalize_or_zero();

    for (mut gun_transform, mut animation, _weapon, ads) in gun_query.iter_mut() {
        update_animation_timers(&mut animation, &time);
        let ads_factor = calculate_ads_factor(ads, 0.99);

        let target_offset =
            calculate_target_offset(&animation, &movement_dir, speed, ads, ads_factor);
        smooth_offset(&mut animation, target_offset);

        apply_idle_animation(&mut animation, speed, ads);
        apply_gun_transform(&mut gun_transform, &camera_transform, &animation);
        apply_gun_rotation(
            &mut gun_transform,
            &camera_transform,
            &movement_dir,
            speed,
            ads,
        );

        //apply_gun_running(&mut gun_transform, &camera_transform, &movement_dir, speed);
    }
}

fn update_velocity(
    state: &mut ResMut<GunAnimationState>,
    player_transform: &Transform,
    time: &Time,
) {
    let current_position = player_transform.translation;
    if state.last_player_position != Vec3::ZERO {
        state.velocity = (current_position - state.last_player_position) / time.delta_secs();
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
    movement_dir: &Vec3,
    speed: f32,
    ads: &ADS,
    ads_factor: f32,
) -> Vec3 {
    if ads.is_ads {
        return Vec3::ZERO;
    }

    if speed > 0.1 && speed < 30.0 {
        calculate_movement_offset(animation, movement_dir, speed, ads_factor)
    } else {
        Vec3::ZERO
    }
}

fn calculate_movement_offset(
    animation: &GunAnimation,
    movement_dir: &Vec3,
    speed: f32,
    ads_factor: f32,
) -> Vec3 {
    let sideways =
        movement_dir.cross(Vec3::Y).x * animation.wobble.intensity * speed * 0.5 * ads_factor;

    let up_down =
        animation.bob.phase.sin() * animation.bob.bob_intensity * speed * 0.5 * ads_factor;

    let forward = movement_dir.z * animation.wobble.intensity * speed * 0.3 * ads_factor;

    Vec3::new(sideways, up_down, forward)
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
        animation.wobble.current_offset = animation.wobble.current_offset.lerp(idle_offset, 0.05);
    }
}

fn calculate_idle_offset(animation: &GunAnimation) -> Vec3 {
    Vec3::new(
        (animation.wobble.time * 0.5).sin() * 0.005,
        (animation.wobble.time * 0.8).sin() * 0.003,
        0.0,
    )
}

fn apply_gun_transform(
    gun_transform: &mut Mut<Transform>,
    camera_transform: &Transform,
    animation: &GunAnimation,
) {
    let final_offset = animation.wobble.base_offset + animation.wobble.current_offset;
    gun_transform.translation =
        camera_transform.translation + (camera_transform.rotation * final_offset);
}

fn apply_gun_rotation(
    gun_transform: &mut Mut<Transform>,
    camera_transform: &Transform,
    movement_dir: &Vec3,
    speed: f32,
    ads: &ADS,
) {
    if speed > 0.1 && !ads.is_ads && speed < 30.0 {
        let tilt = Quat::from_rotation_z(movement_dir.x * 0.1 * speed.min(1.0));
        let pitch = Quat::from_rotation_x(-movement_dir.z * 0.05 * speed.min(1.0));
        gun_transform.rotation = camera_transform.rotation * tilt * pitch;
    } else {
        gun_transform.rotation = camera_transform.rotation;
    }
}

/*
fn apply_gun_running(
    gun_transform: &mut Mut<Transform>,
    camera_transform: &Transform,
    movement_dir: &Vec3,
    speed: f32,
) {
    if speed > 30.0 {
        let tilt = Quat::from_rotation_z(movement_dir.x * 3.0 * speed.min(1.0));
        let pitch = Quat::from_rotation_x(-movement_dir.z * 5.0 * speed.min(1.0));
        gun_transform.rotation = camera_transform.rotation * tilt * pitch;
    } else {
        gun_transform.rotation = camera_transform.rotation;
    }
}
*/
