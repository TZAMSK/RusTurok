use bevy::ecs::system::Commands;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    camera::{components::CameraSensitivity, renderlayers::VIEW_MODEL_RENDER_LAYER},
    player::components::Player,
    weapons::systems::spawn_weapon,
};

use super::components::{FirstLayerCamera, SecondLayerCamera};

pub fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::new(),
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        Children::spawn((
            spawn_main_camera(),
            spawn_view_model_camera(),
            spawn_weapon(asset_server),
        )),
    ));
}

fn spawn_main_camera() -> Spawn<(FirstLayerCamera, Camera, Camera3d, Projection)> {
    Spawn((
        FirstLayerCamera,
        Camera {
            order: 1,
            ..default()
        },
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 140.0_f32.to_radians(),
            ..default()
        }),
    ))
}

fn spawn_view_model_camera() -> Spawn<(
    SecondLayerCamera,
    Camera,
    Camera3d,
    Projection,
    RenderLayers,
)> {
    Spawn((
        SecondLayerCamera,
        Camera {
            order: 2,
            ..default()
        },
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 80.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
    ))
}
