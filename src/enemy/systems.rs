use super::components::*;
use crate::combat::{DamageMessage, HitDetection};
use crate::player::components::Player;
use crate::weapons::components::weapon::Weapon;
use bevy::prelude::*;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    const GOLDEN_ANGLE: f32 = 137.507_77;

    let mut hsla = Hsla::hsl(0.0, 1.0, 0.5);
    for x in -1..5 {
        for y in -1..5 {
            commands.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(materials.add(Color::from(hsla))),
                Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                Enemy::new(),
            ));
            hsla = hsla.rotate_hue(GOLDEN_ANGLE);
        }
    }
}

pub fn apply_damage_to_enemies(
    mut commands: Commands,
    mut hit_detection: ResMut<HitDetection>,
    mut damage_events: MessageReader<DamageMessage>,
    mut enemy_query: Query<&mut Enemy>,
    mut player_query: Query<&mut Player>,
    mut weapon_query: Query<&mut Weapon>,
) {
    for event in damage_events.read() {
        let Ok(mut enemy) = enemy_query.get_mut(event.target) else {
            continue;
        };

        let Ok(mut weapon) = weapon_query.single_mut() else {
            continue;
        };

        enemy.health -= event.amount;
        hit_detection.hit = true;

        if enemy.health <= 0.0 {
            commands.entity(event.target).despawn();
            weapon.unique_trait.stats.add_kill();
            hit_detection.killed = true;

            if let Some(shooter) = event.shooter {
                if let Ok(mut player) = player_query.get_mut(shooter) {
                    player.add_xp(enemy.xp);
                }
            }
        }
    }
}
