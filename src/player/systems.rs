use std::f32::consts::FRAC_2_PI;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    camera::components::CameraSensitivity,
    player::components::Player,
    weapons::components::{Weapon, ADS},
};

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
    mut weapon_query: Query<&mut ADS, With<Weapon>>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut player_transform)) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;
        let gravity = -9.81;
        let mut speed = player.movement.speed;

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
            player.movement.is_sprinting = true;
            player.movement.is_crouching = false;
            player.movement.is_sliding = false;

            if player.movement.is_sprinting {
                for mut ads in weapon_query.iter_mut() {
                    ads.is_ads = false;
                    ads.ads_progress = 0.0;
                }
            }
        }

        if player.movement.is_sprinting && direction != Vec3::ZERO {
            speed *= 1.22;
            for mut ads in weapon_query.iter_mut() {
                ads.is_ads = false;
                continue;
            }
        }

        //Slide and crouch
        if keyboard_input.just_pressed(KeyCode::ControlLeft) {
            if player.movement.is_sprinting {
                player.movement.is_sliding = true;
                player.movement.is_crouching = false;

                player.movement.slide_direction =
                    Vec3::new(horizontal_forward.x, 0.0, horizontal_forward.z).normalize();
            } else {
                player.movement.is_crouching = !player.movement.is_crouching;
            }

            player.movement.is_sprinting = false;
        }

        if direction != Vec3::ZERO {
            if player.movement.is_sliding {
                speed *= 1.22;
            }

            if player.movement.is_crouching {
                speed *= 0.22;
            }
        }

        if direction == Vec3::ZERO {
            player.movement.is_sprinting = false;
            player.movement.is_sliding = false;
        }

        if direction.length() > 0.0 && !player.movement.is_sliding {
            player_transform.translation += direction.normalize() * speed * time.delta_secs();
        }

        if player.movement.is_sliding {
            player_transform.translation +=
                player.movement.slide_direction * speed * time.delta_secs();
        }

        //Jump
        if keyboard_input.just_pressed(KeyCode::Space) && player.movement.is_grounded {
            player.movement.velocity = player.movement.jump_height;
            player.movement.is_grounded = false;
            player.movement.is_sliding = false;
        }

        if !player.movement.is_grounded {
            player.movement.velocity += gravity * time.delta_secs();
            player_transform.translation.y += player.movement.velocity * time.delta_secs();

            if player_transform.translation.y <= 0.0 {
                player_transform.translation.y = 0.0;
                player.movement.velocity = 0.0;
                player.movement.is_grounded = true;
            }
        }
    }
}

pub fn ground_detection_system(mut player_query: Query<(&mut Player, &Transform), With<Player>>) {
    for (mut player, transform) in player_query.iter_mut() {
        let ground_level = 0.0;

        if transform.translation.y <= ground_level + 0.1 && player.movement.velocity <= 0.0 {
            player.movement.is_grounded = true;
        } else {
            player.movement.is_grounded = false;
        }
    }
}
