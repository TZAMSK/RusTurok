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
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT * 2.0, PITCH_LIMIT * 2.0);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut player_transform)) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;
        let gravity = -9.81;
        let mut speed = player.speed;

        let forward = player_transform.forward();
        let right = player_transform.right();

        let horizontal_forward = Vec3::new(forward.x, 0.0, forward.z);
        let horizontal_right = Vec3::new(right.x, 0.0, right.z);

        //WASD
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

        //Sprint
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
            player.is_sprinting = !player.is_sprinting;
        }

        if direction == Vec3::ZERO {
            player.is_sprinting = false;
        }

        if player.is_sprinting && direction != Vec3::ZERO {
            speed *= 1.22;
        }

        //Jump
        if direction.length() > 0.0 {
            player_transform.translation += direction.normalize() * speed * time.delta_secs();
        }

        if keyboard_input.just_pressed(KeyCode::Space) && player.is_grounded {
            player.velocity = player.jump_height;
            player.is_grounded = false;
        }

        if !player.is_grounded {
            player.velocity += gravity * time.delta_secs();
            player_transform.translation.y += player.velocity * time.delta_secs();

            if player_transform.translation.y <= 0.0 {
                player_transform.translation.y = 0.0;
                player.velocity = 0.0;
                player.is_grounded = true;
            }
        }
    }
}

pub fn ground_detection_system(mut player_query: Query<(&mut Player, &Transform), With<Player>>) {
    for (mut player, transform) in player_query.iter_mut() {
        let ground_level = 0.0;

        if transform.translation.y <= ground_level + 0.1 && player.velocity <= 0.0 {
            player.is_grounded = true;
        } else {
            player.is_grounded = false;
        }
    }
}
