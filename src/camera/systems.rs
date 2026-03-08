use super::components::FirstLayerCamera;
use crate::{camera::components::CameraSensitivity, player::components::Player};
use bevy::ecs::system::Commands;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Player::new(),
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.93, 0.0),
            Visibility::default(),
        ))
        .with_child((
            FirstLayerCamera,
            Camera {
                order: 0,
                ..default()
            },
            Camera3d::default(),
        ));
}
