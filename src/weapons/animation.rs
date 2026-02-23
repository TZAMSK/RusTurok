use crate::{camera::components::FirstLayerCamera, player::components::Player};
use bevy::prelude::*;

use super::components::{GunAnimation, Weapon};

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
    mut gun_query: Query<(&mut Transform, &mut GunAnimation, &mut Weapon), Without<Player>>,
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

    // Update velocity
    let current_position = player_transform.translation;
    if animation_state.last_player_position != Vec3::ZERO {
        animation_state.velocity =
            (current_position - animation_state.last_player_position) / time.delta_secs();
    }
    animation_state.last_player_position = current_position;

    let speed = animation_state.velocity.length();

    for (mut gun_transform, mut animation, _weapon) in gun_query.iter_mut() {
        animation.wobble.time += time.delta_secs();
        animation.bob.phase += time.delta_secs() * animation.bob.bob_speed;

        let movement_dir = animation_state.velocity.normalize_or_zero();

        // Calculate target offset based on movement
        let target_offset = if speed > 0.1 {
            let sideways = movement_dir.cross(Vec3::Y).x * animation.wobble.intensity * speed * 0.5;
            let up_down = animation.bob.phase.sin() * animation.bob.bob_intensity * speed * 0.5;
            let forward = movement_dir.z * animation.wobble.intensity * speed * 0.3;

            Vec3::new(sideways, up_down, forward)
        } else {
            Vec3::ZERO
        };

        // Smoothly interpolate to target offset
        animation.wobble.current_offset = animation
            .wobble
            .current_offset
            .lerp(target_offset, animation.wobble.smoothness);

        // Gentle idle breathing when standing still
        if speed < 0.1 {
            let idle_offset = Vec3::new(
                (animation.wobble.time * 0.5).sin() * 0.005,
                (animation.wobble.time * 0.8).sin() * 0.003,
                0.0,
            );
            animation.wobble.current_offset =
                animation.wobble.current_offset.lerp(idle_offset, 0.05);
        }

        // Final gun position relative to camera
        let final_offset = animation.wobble.base_offset + animation.wobble.current_offset;
        gun_transform.translation =
            camera_transform.translation + camera_transform.rotation * final_offset;

        // Apply tilt when moving
        if speed > 0.1 {
            let tilt = Quat::from_rotation_z(movement_dir.x * 0.1 * speed.min(1.0));
            let pitch = Quat::from_rotation_x(-movement_dir.z * 0.05 * speed.min(1.0));
            gun_transform.rotation = camera_transform.rotation * tilt * pitch;
        } else {
            gun_transform.rotation = camera_transform.rotation;
        }
    }
}
