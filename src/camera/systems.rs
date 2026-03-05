use super::components::{FirstLayerCamera, SecondLayerCamera};
use crate::{
    camera::{components::CameraSensitivity, renderlayers::VIEW_MODEL_RENDER_LAYER},
    player::components::Player,
};
use bevy::ecs::system::Commands;
use bevy::{camera::visibility::RenderLayers, prelude::*};

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Player::new(),
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.93, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                FirstLayerCamera,
                Camera {
                    order: 0,
                    ..default()
                },
                Camera3d::default(),
                Projection::Perspective(PerspectiveProjection {
                    fov: 120.0_f32.to_radians(),
                    ..default()
                }),
            ));

            parent.spawn((
                SecondLayerCamera,
                Camera {
                    order: 1,
                    ..default()
                },
                Camera3d::default(),
                Projection::Perspective(PerspectiveProjection {
                    fov: 80.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}
