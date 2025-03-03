use bevy::{prelude::*, render::view::RenderLayers};

use crate::camera::renderlayers::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

use crate::world::components::World;

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let material = materials.add(Color::WHITE);

    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone()), World));
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 100.0,
            range: 10.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, -4.0, -0.8),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}
