use super::{
    bullets::DespawnAfter,
    components::{Bullet, BulletTracer},
};
use crate::{
    camera::components::FirstLayerCamera, enemy::components::Enemy, player::components::Player,
};
use bevy::{color::palettes::tailwind, prelude::*};

struct RaycastHit {
    point: Vec3,
    entity: Option<Entity>,
    distance: f32,
}

pub fn spawn_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<&mut Player>,
    weapon_query: Query<&GlobalTransform, With<BulletTracer>>,
    camera_query: Query<&GlobalTransform, (With<Camera>, With<FirstLayerCamera>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    enemy_query: Query<(Entity, &GlobalTransform), With<Enemy>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Ok(tracer_transform) = weapon_query.single() else {
                return;
            };
            let Ok(camera_transform) = camera_query.single() else {
                return;
            };

            let enable_dmg = keyboard.pressed(KeyCode::KeyQ);

            let camera_direction = camera_transform.forward().normalize();
            let camera_start = camera_transform.translation();

            let weapon_start = tracer_transform.translation();

            let max_distance = 1000.0;
            let hit =
                raycast_from_camera(camera_start, camera_direction, max_distance, &enemy_query);

            if enable_dmg {
                if let Some(enemy_entity) = hit.entity {
                    commands.entity(enemy_entity).despawn();
                    player.add_xp(10.0);
                }
            }

            let weapon_to_hit = (hit.point - weapon_start).normalize();

            spawn_visual_tracer(
                &mut commands,
                &mut meshes,
                &mut materials,
                weapon_start,
                weapon_to_hit,
                hit.point,
                &time,
            );

            spawn_muzzle_flash(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                weapon_start,
                camera_direction,
                &time,
            );

            spawn_impact_effect(
                &mut commands,
                &mut meshes,
                &mut materials,
                hit.point,
                hit.entity.is_some(),
                &time,
            );
        }
    }
}

fn raycast_from_camera(
    start: Vec3,
    direction: Vec3,
    max_distance: f32,
    enemy_query: &Query<(Entity, &GlobalTransform), With<Enemy>>,
) -> RaycastHit {
    let mut closest_hit: Option<(Entity, Vec3, f32)> = None;

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation();
        let enemy_radius = 0.5;

        let to_enemy = enemy_pos - start;
        let t = to_enemy.dot(direction);

        if t > 0.0 && t < max_distance {
            let closest_point_on_ray = start + direction * t;
            let distance_to_enemy = closest_point_on_ray.distance(enemy_pos);

            if distance_to_enemy < enemy_radius {
                let hit_distance = t
                    - (enemy_radius * enemy_radius - distance_to_enemy * distance_to_enemy)
                        .sqrt()
                        .max(0.0);
                let hit_point = start + direction * hit_distance;

                if closest_hit.is_none() || hit_distance < closest_hit.unwrap().2 {
                    closest_hit = Some((enemy_entity, hit_point, hit_distance));
                }
            }
        }
    }

    match closest_hit {
        Some((entity, point, distance)) => RaycastHit {
            point,
            entity: Some(entity),
            distance,
        },
        None => RaycastHit {
            point: start + direction * max_distance,
            entity: None,
            distance: max_distance,
        },
    }
}

fn spawn_visual_tracer(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    direction: Vec3,
    hit_point: Vec3,
    time: &Res<Time>,
) {
    let distance = start.distance(hit_point);

    let tracer_length = distance;
    let tracer_mesh = meshes.add(Cylinder::new(0.03, tracer_length));
    let tracer_material = materials.add(StandardMaterial {
        base_color: Color::from(tailwind::YELLOW_500),
        emissive: Color::from(tailwind::YELLOW_500).into(),
        ..default()
    });

    let mid_point = (start + hit_point) / 2.0;

    commands.spawn((
        Mesh3d(tracer_mesh),
        MeshMaterial3d(tracer_material),
        Transform {
            translation: mid_point,
            rotation: Quat::from_rotation_arc(Vec3::Y, direction),
            ..default()
        },
        Bullet,
        DespawnAfter(time.elapsed_secs() + 0.05),
    ));
}

fn spawn_muzzle_flash(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    direction: Vec3,
    time: &Res<Time>,
) {
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
            emissive: Color::from(tailwind::YELLOW_500).into(),
            ..default()
        });

        commands.spawn((
            Mesh3d(flash_mesh.clone()),
            MeshMaterial3d(flash_material),
            Transform {
                translation: position,
                rotation: Quat::from_rotation_arc(rotation_axis, direction),
                ..default()
            },
            DespawnAfter(time.elapsed_secs() + 0.05),
        ));
    }
}

fn spawn_impact_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    hit_enemy: bool,
    time: &Res<Time>,
) {
    let color = if hit_enemy {
        tailwind::RED_500
    } else {
        tailwind::GRAY_500
    };

    let impact_mesh = meshes.add(Sphere::new(0.1).mesh().uv(32, 18));
    let impact_material = materials.add(StandardMaterial {
        base_color: Color::from(color),
        emissive: Color::from(color).into(),
        ..default()
    });

    commands.spawn((
        Mesh3d(impact_mesh),
        MeshMaterial3d(impact_material),
        Transform::from_translation(position),
        DespawnAfter(time.elapsed_secs() + 0.1),
    ));
}
