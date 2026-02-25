use super::components::{FirstLayerCamera, SecondLayerCamera};
use crate::{
    camera::{components::CameraSensitivity, renderlayers::VIEW_MODEL_RENDER_LAYER},
    player::components::Player,
    weapons::components::BulletTracer,
    weapons::components::{GunAnimation, PrimaryWeaponType, Weapon, WeaponType, ADS},
};
use bevy::ecs::system::Commands;
use bevy::{camera::visibility::RenderLayers, prelude::*};

/*
pub fn spawn_tool_camera(mut commands: Commands) {
    commands.spawn((
        Camera {
            order: 0,
            ..default()
        },
        Camera3d::default(),
    ));
}
*/

pub fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
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

            spawn_weapon_as_child(parent, &asset_server);
        });
}

fn spawn_weapon_as_child(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>) {
    let hip_position = Vec3::new(0.26, -0.35, 0.0);
    let ads_position = Vec3::new(0.0, -0.279, 0.094);

    parent
        .spawn((
            SceneRoot(asset_server.load("models/ak.glb#Scene0")),
            Transform::from_xyz(hip_position.x, hip_position.y, hip_position.z),
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            Weapon::new(
                "a gun".to_string(),
                WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
            ),
            GunAnimation::default(),
            ADS::new(hip_position, ads_position),
        ))
        .with_children(|parent| {
            parent.spawn((
                Transform {
                    translation: Vec3::new(0.0, 0.0952, -1.440),
                    ..default()
                },
                BulletTracer,
            ));
        });
}
