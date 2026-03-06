use crate::{
    camera::components::FirstLayerCamera, combat::DamageMessage, enemy::components::Enemy,
    weapons::bullets::DespawnAfter,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct DMGIndicator {
    pub enemy: Entity,
    pub animation_progress: f32,
    pub animation_complete: bool,
    pub animation_speed: f32,
    pub base_offset: Vec2,
    pub drift_right: bool,
}

pub fn dmg_indicator_spawn(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, &GlobalTransform)>,
    mut damage_events: MessageReader<DamageMessage>,
    camera_query: Query<(&Camera, &GlobalTransform), With<FirstLayerCamera>>,
    time: Res<Time>,
) {
    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };
    for event in damage_events.read() {
        let Ok((_enemy, enemy_transform)) = enemy_query.get(event.target) else {
            continue;
        };
        let world_position = enemy_transform.translation();
        let Ok(viewport_position) =
            camera.world_to_viewport(camera_global_transform, world_position)
        else {
            continue;
        };
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(viewport_position.y),
                left: px(viewport_position.x),
                ..default()
            },
            DMGIndicator {
                enemy: event.target,
                animation_progress: 0.0,
                animation_complete: false,
                animation_speed: 3.0,
                base_offset: Vec2::new(80.0, 40.0),
                drift_right: rand::random(),
            },
            children![(
                Text::new(format!("{}", event.amount as u32)),
                TextColor(Color::srgba(
                    133.0 / 255.0,
                    100.0 / 255.0,
                    90.0 / 255.0,
                    1.0,
                )),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::ZERO,
                    ..default()
                },
                TextLayout::default().with_no_wrap(),
            )],
            DespawnAfter(time.elapsed_secs() + 0.3),
        ));
    }
}

pub fn update_dmg_indicator(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform), With<FirstLayerCamera>>,
    mut labels: Query<(Entity, &mut Node, &mut DMGIndicator)>,
    labeled: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };
    for (label_entity, mut node, mut label) in &mut labels {
        let Ok(world_transform) = labeled.get(label.enemy) else {
            commands.entity(label_entity).despawn();
            continue;
        };
        label.animation_progress =
            (label.animation_progress + label.animation_speed * time.delta_secs()).clamp(0.0, 1.0);
        let world_position = world_transform.translation();
        let Ok(viewport_position) =
            camera.world_to_viewport(camera_global_transform, world_position)
        else {
            continue;
        };
        let offset_y = label.base_offset.y * label.animation_progress;
        let offset_x = label.base_offset.x * label.animation_progress;
        node.top = px(viewport_position.y - offset_y);
        if label.drift_right {
            node.left = px(viewport_position.x + offset_x);
        } else {
            node.left = px(viewport_position.x - offset_x);
        }
    }
}
