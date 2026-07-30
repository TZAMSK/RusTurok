use bevy::prelude::*;

use crate::shop::components::Shop;

pub fn spawn_shop(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut hsla = Hsla::hsl(0.0, 1.0, 0.5);

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::from(hsla))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Shop::new(),
    ));
}
