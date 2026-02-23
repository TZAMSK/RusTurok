use bevy::ecs::system::Commands;
use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    camera::{components::CameraSensitivity, renderlayers::VIEW_MODEL_RENDER_LAYER},
    player::components::Player,
    weapons::components::BulletTracer,
    weapons::components::{GunAnimation, PrimaryWeaponType, Weapon, WeaponType, ADS},
};

use super::components::{FirstLayerCamera, SecondLayerCamera};

pub fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Player::new(),
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Spawn main camera
            parent.spawn((
                FirstLayerCamera,
                Camera {
                    order: 0,
                    ..default()
                },
                Camera3d::default(),
                Projection::Perspective(PerspectiveProjection {
                    fov: 140.0_f32.to_radians(),
                    ..default()
                }),
            ));

            // Spawn view model camera
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

            // Spawn weapon directly as child
            spawn_weapon_as_child(parent, &asset_server);
        });
}

fn spawn_weapon_as_child(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>) {
    let hip_position = Vec3::new(0.0, 0.05, 0.7);
    let ads_position = Vec3::new(-0.531, 0.3, 0.6);

    parent
        .spawn((
            SceneRoot(asset_server.load("models/ak.glb#Scene0")),
            Transform::from_xyz(0.0, 0.05, 0.7),
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            Weapon::new(
                "a gun".to_string(),
                WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
            ),
            GunAnimation::default(),
            ADS::new(hip_position, ads_position),
        ))
        .with_children(|parent| {
            // Bullet tracer as child
            parent.spawn((
                Transform {
                    translation: Vec3::new(0.53, -0.46, -2.15),
                    ..default()
                },
                BulletTracer,
            ));

            // Scope as child
            parent.spawn((
                SceneRoot(asset_server.load("models/redot.glb#Scene0")),
                Transform {
                    translation: Vec3::new(0.602, -0.53, -1.2),
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    scale: Vec3::new(0.15, 0.15, 0.15),
                },
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}
