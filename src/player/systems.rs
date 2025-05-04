use std::f32::consts::FRAC_2_PI;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{camera::components::CameraSensitivity, player::components::Player};
pub fn move_player_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_2_PI;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT * 1.0, PITCH_LIMIT * 2.0);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player, mut player_transform)) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        let forward = player_transform.forward();
        let right = player_transform.right();

        let horizontal_forward = Vec3::new(forward.x, 0.0, forward.z);
        let horizontal_right = Vec3::new(right.x, 0.0, right.z);

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += horizontal_forward;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= horizontal_right;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= horizontal_forward;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += horizontal_right;
        }

        if direction.length() > 0.0 {
            player_transform.translation +=
                direction.normalize() * player.speed * time.delta_secs();
        }
    }
}
