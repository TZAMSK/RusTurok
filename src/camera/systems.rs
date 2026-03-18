use super::components::FirstLayerCamera;
use crate::camera::components::WeaponLayerCamera;
use crate::camera::renderlayers::{VIEW_MODEL_RENDER_LAYER, WORLD_RENDER_LAYER};
use crate::{camera::components::CameraSensitivity, player::components::Player};
use bevy::camera::visibility::RenderLayers;
use bevy::camera::Camera3dDepthLoadOp;
use bevy::prelude::*;

pub const FIRST_LAYER_HIP_FOV: f32 = 120.0;
pub const FIRST_LAYER_ADS_FOV: f32 = 70.0;
pub const WEAPON_LAYER_FOV: f32 = 40.0;

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Player::new(),
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                FirstLayerCamera,
                Camera3d::default(),
                Camera {
                    order: 0,
                    ..default()
                },
                Projection::Perspective(PerspectiveProjection {
                    fov: FIRST_LAYER_HIP_FOV.to_radians(),
                    ..default()
                }),
                Transform::default(),
                RenderLayers::layer(WORLD_RENDER_LAYER),
            ));

            parent.spawn((
                WeaponLayerCamera,
                Camera3d {
                    depth_load_op: Camera3dDepthLoadOp::Clear(0.0),
                    ..default()
                },
                Camera {
                    order: 1,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                Projection::Perspective(PerspectiveProjection {
                    fov: WEAPON_LAYER_FOV.to_radians(),
                    near: 0.001,
                    ..default()
                }),
                Transform::default(),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}
