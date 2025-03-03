use ::bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
};

use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};

use crate::{
    camera::{
        components::{CameraSensitivity, WorldModelCamera},
        renderlayers::VIEW_MODEL_RENDER_LAYER,
    },
    player::components::Player,
};

pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 120.0_f32.to_radians(),
                    ..default()
                }),
            ));

            parent.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 80.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                NotShadowCaster,
            ));
        });
}
