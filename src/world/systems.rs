use bevy::{prelude::*, render::view::RenderLayers};

use crate::camera::renderlayers::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));
    let black_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        reflectance: 0.9,
        perceptual_roughness: 0.8,
        ..default()
    });

    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        reflectance: 0.3,
        perceptual_roughness: 0.8,
        ..default()
    });

    for x in -4..4 {
        for z in -30..0 {
            commands.spawn((
                Mesh3d(plane_mesh.clone()),
                MeshMaterial3d(if (x + z) % 2 == 0 {
                    black_material.clone()
                } else {
                    white_material.clone()
                }),
                Transform::from_xyz(x as f32 * 2.0, -1.0, z as f32 * 2.0),
            ));
        }
    }
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}
