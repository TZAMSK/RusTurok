use super::components::*;
use bevy::prelude::*;

const NUMBER_ENEMIES: u32 = 19;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    const GOLDEN_ANGLE: f32 = 137.507_77;

    let mut hsla = Hsla::hsl(0.0, 1.0, 0.5);
    for x in -1..2 {
        for z in -1..2 {
            commands.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(materials.add(Color::from(hsla))),
                Transform::from_translation(Vec3::new(x as f32, 0.0, z as f32)),
                Enemy::new(),
            ));
            hsla = hsla.rotate_hue(GOLDEN_ANGLE);
        }
    }
}
