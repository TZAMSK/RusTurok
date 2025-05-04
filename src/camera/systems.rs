use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
};

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    camera::{
        components::{CameraSensitivity, WorldModelCamera},
        renderlayers::VIEW_MODEL_RENDER_LAYER,
    },
    player::components::Player,
    weapons::systems::spawn_weapon,
};

pub fn spawn_camera(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player::new(),
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        Children::spawn((
            Spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 140.0_f32.to_radians(),
                    ..default()
                }),
            )),
            spawn_view_model_camera(),
            spawn_weapon(meshes, materials),
        )),
    ));
}

fn spawn_view_model_camera() -> Spawn<(Camera3d, Projection, RenderLayers)> {
    Spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 80.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
    ))
}
