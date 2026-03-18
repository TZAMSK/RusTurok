use bevy::{
    camera::visibility::RenderLayers,
    light::{FogVolume, VolumetricLight},
    prelude::*,
};

use crate::camera::{
    components::{FirstLayerCamera, WeaponLayerCamera},
    renderlayers::{VIEW_MODEL_RENDER_LAYER, WORLD_RENDER_LAYER},
};

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

pub fn spawn_sun_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            color: Color::srgb(255.0 / 255.0, 224.0 / 255.0, 102.0 / 255.0),
            ..default()
        },
        Transform::from_xyz(100.0, 200.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[WORLD_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

pub fn spawn_fog(
    mut commands: Commands,
    camera_query: Query<Entity, (With<FirstLayerCamera>, With<Camera3d>)>,
    weapon_camera_query: Query<Entity, (With<WeaponLayerCamera>, With<Camera3d>)>,
) {
    let Ok(cam_entity) = camera_query.single() else {
        return;
    };

    let Ok(weap_cam_entity) = weapon_camera_query.single() else {
        return;
    };

    let fog = DistanceFog {
        color: Color::srgba(0.35, 0.48, 0.66, 1.0),
        falloff: FogFalloff::from_visibility_colors(
            15.0,
            Color::srgb(232.0 / 255.0, 204.0 / 255.0, 255.0 / 255.0),
            Color::srgba(0.8, 0.844, 1.0, 0.3),
        ),
        ..default()
    };

    commands.entity(cam_entity).insert(fog.clone());
    commands.entity(weap_cam_entity).insert(fog);
}
