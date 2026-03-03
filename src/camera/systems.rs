use super::components::{FirstLayerCamera, SecondLayerCamera};
use crate::{
    camera::{components::CameraSensitivity, renderlayers::VIEW_MODEL_RENDER_LAYER},
    player::components::Player,
    weapons::{
        components::{BulletTracer, GunAnimation, PrimaryWeaponType, Weapon, WeaponType, ADS},
        transition::{WeaponAnimationStance, WeaponAnimationState},
    },
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
    let initial_weapon_state =
        WeaponAnimationState::define_state_by_stance(WeaponAnimationStance::Grounded);

    let ads_position = Vec3::new(0.0, -0.279, 0.094);

    parent
        .spawn((
            SceneRoot(asset_server.load("models/safeak2/ak6.glb#Scene0")),
            Transform::from_xyz(
                initial_weapon_state.translation.x,
                initial_weapon_state.translation.y,
                initial_weapon_state.translation.z,
            ),
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            Weapon::new(
                "a gun".to_string(),
                WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
            ),
            GunAnimation::default(),
            initial_weapon_state,
            ADS::new(initial_weapon_state.translation, ads_position),
            AnimationPlayer::default(),
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
