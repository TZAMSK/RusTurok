use bevy::{color::palettes::tailwind, prelude::*, render::view::RenderLayers};

use crate::{camera::renderlayers::VIEW_MODEL_RENDER_LAYER, player::components::Player};

use super::components::{Bullet, PrimaryWeaponType, Weapon, WeaponType};

pub fn spawn_weapon(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Spawn<(
    Mesh3d,
    MeshMaterial3d<StandardMaterial>,
    Transform,
    RenderLayers,
    Weapon,
)> {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    Spawn((
        Mesh3d(arm),
        MeshMaterial3d(arm_material),
        Transform::from_xyz(0.2, -0.1, -0.25),
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        Weapon::new(
            "a gun".to_string(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
        ),
    ))
}

pub fn spawn_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Sphere::new(0.02));
    let arm_material = materials.add(Color::from(tailwind::ORANGE_400));

    if mouse_input.pressed(MouseButton::Left) {
        if let Some(player) = player_query.iter().next() {
            let player_pos = player.translation;
            let player_rotation = player.rotation;

            commands.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform {
                    translation: Vec3::new(player_pos.x, player_pos.y - 0.1, player_pos.z + 1.),
                    rotation: player_rotation,
                    ..default()
                },
                Bullet,
            ));
        }
    }
}

pub fn bullet_movement(
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
    weapon: Query<&Weapon, With<Weapon>>,
    time: Res<Time>,
) {
    let weapon = weapon.single().unwrap();
    for mut bullet_transform in bullet_query.iter_mut() {
        let direction = bullet_transform.rotation.mul_vec3(Vec3::NEG_Z);

        bullet_transform.translation +=
            direction.normalize() * weapon.unique_trait.bullet_speed * time.delta_secs();
    }
}
