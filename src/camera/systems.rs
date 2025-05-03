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
    commands
        .spawn((
            Player::new(),
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 140.0_f32.to_radians(),
                    ..default()
                }),
            ));

            spawn_view_model_camera(parent);
            spawn_weapon(parent, meshes, materials);
        });
}

fn spawn_view_model_camera(parent: &mut impl ChildBuild) {
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
}
