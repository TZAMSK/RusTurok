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

        const PITCH_LIMIT: f32 = FRAC_2_PI - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player, mut player_transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += *player_transform.forward()
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= *player_transform.right()
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= *player_transform.forward()
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += *player_transform.right()
        }

        if direction.length() > 0.0 {
            player_transform.translation +=
                direction.normalize() * player.speed * time.delta_secs();
        }
    }
}
