use bevy::{color::palettes::tailwind, prelude::*};

use crate::{camera::components::FirstLayerCamera, enemy::components::Enemy};

use super::{
    bullets::DespawnAfter,
    components::{Bullet, BulletDirection, BulletTracer, Weapon},
};

pub fn spawn_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    trace_query: Query<&GlobalTransform, With<BulletTracer>>,
    camera_query: Query<&GlobalTransform, (With<Camera>, With<FirstLayerCamera>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
        let arm_material = materials.add(Color::from(tailwind::YELLOW_500));

        let tracer_transform = trace_query.single().unwrap();
        let camera_transform = camera_query.single().unwrap();

        let start = tracer_transform.translation();
        let direction = camera_transform.forward().normalize();

        commands.spawn((
            Mesh3d(arm),
            MeshMaterial3d(arm_material),
            Transform {
                translation: start,
                rotation: Quat::from_rotation_arc(Vec3::NEG_Z, direction),
                ..default()
            },
            Bullet,
            BulletDirection(direction),
        ));

        for i in 1..=2 {
            let rotation_axis: Vec3 = match i {
                1 => Vec3::X,
                2 => Vec3::Z,
                _ => Vec3::Y,
            };
            let flash_mesh = meshes.add(Plane3d::default().mesh().size(1.8, 1.8));
            let flash_material = materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("models/muzzle_flash.png")),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });
            commands.spawn((
                Mesh3d(flash_mesh),
                MeshMaterial3d(flash_material),
                Transform {
                    translation: start,
                    rotation: Quat::from_rotation_arc(rotation_axis, direction),
                    ..default()
                },
                DespawnAfter(time.elapsed_secs() + 0.05),
            ));
        }
    }
}

pub fn bullet_movement(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
    weapon: Query<&Weapon, With<Weapon>>,
    time: Res<Time>,
) {
    let weapon = weapon.single().unwrap();

    for (mut transform, direction) in bullet_query.iter_mut() {
        transform.translation += direction.0 * weapon.unique_trait.bullet_speed * time.delta_secs();
    }
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);
            let bullet_radius = 0.01;
            let enemy_radius = 0.25;

            if distance < bullet_radius + enemy_radius {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
