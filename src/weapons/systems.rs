use bevy::{
    color::palettes::tailwind, ecs::spawn::SpawnRelatedBundle, prelude::*,
    render::view::RenderLayers,
};

use crate::camera::{components::FirstLayerCamera, renderlayers::VIEW_MODEL_RENDER_LAYER};

use super::components::{
    Bullet, BulletDirection, BulletTracer, PrimaryWeaponType, Weapon, WeaponType,
};

pub fn spawn_weapon(
    asset_server: Res<AssetServer>,
) -> Spawn<(
    SceneRoot,
    Transform,
    RenderLayers,
    Weapon,
    SpawnRelatedBundle<ChildOf, Spawn<(Transform, BulletTracer)>>,
)> {
    Spawn((
        SceneRoot(asset_server.load("models/ak.glb#Scene0")),
        Transform::from_xyz(0.1, 0.1, 0.5),
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        Weapon::new(
            "a gun".to_string(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
        ),
        Children::spawn(Spawn((
            Transform {
                translation: Vec3::new(0.53, -0.46, -1.98),
                ..default()
            },
            BulletTracer,
        ))),
    ))
}

pub fn spawn_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    trace_query: Query<&GlobalTransform, With<BulletTracer>>,
    camera_query: Query<&GlobalTransform, (With<Camera>, With<FirstLayerCamera>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let arm = meshes.add(Cuboid::new(0.04, 0.04, 0.1));
    let arm_material = materials.add(Color::from(tailwind::YELLOW_500));

    let tracer_transform = trace_query.single().unwrap();
    let camera_transform = camera_query.single().unwrap();

    let start = tracer_transform.translation();
    let camera_forward = camera_transform.forward().normalize();

    commands.spawn((
        Mesh3d(arm),
        MeshMaterial3d(arm_material),
        Transform {
            translation: start,
            rotation: Quat::from_rotation_arc(Vec3::NEG_Z, camera_forward),
            ..default()
        },
        Bullet,
        BulletDirection(camera_forward),
    ));
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
