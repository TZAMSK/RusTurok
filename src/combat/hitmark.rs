use crate::{combat::HitDetection, weapons::bullets::DespawnAfter};
use bevy::{
    color::palettes::css::{RED, WHITE},
    ecs::relationship::RelatedSpawnerCommands,
    prelude::*,
};

#[derive(Component)]
pub struct Hitmark {
    width: f32,
    gap: f32,
    spawn_time: f32,
}

impl Hitmark {
    pub fn new(spawn_time: f32) -> Self {
        Self {
            width: 30.0,
            gap: 1.0,
            spawn_time,
        }
    }
}

pub fn spawn_hitmark(mut commands: Commands, hit_detection: Res<HitDetection>, time: Res<Time>) {
    if hit_detection.hit {
        let color: Color = if hit_detection.killed {
            RED.into()
        } else {
            WHITE.into()
        };

        let base_width = 30.0_f32;
        let base_gap = 1.0_f32;
        let thickness = 1.0_f32;
        let arm_len = base_width / 2.0 - base_gap;
        let offset = base_gap + arm_len / 2.0;

        let base_node = Node {
            position_type: PositionType::Absolute,
            left: Val::Px(-arm_len / 2.0),
            top: Val::Px(-thickness / 2.0),
            width: Val::Px(arm_len),
            height: Val::Px(thickness),
            ..default()
        };

        let spawn_arm =
            |parent: &mut RelatedSpawnerCommands<ChildOf>, dx: f32, dy: f32, angle: f32| {
                parent.spawn((
                    base_node.clone(),
                    BackgroundColor(color),
                    UiTransform {
                        translation: Val2::new(Val::Px(dx), Val::Px(dy)),
                        rotation: Rot2::radians(angle),
                        scale: Vec2::ONE,
                    },
                ));
            };

        let angle_45 = std::f32::consts::FRAC_PI_4;
        let neg_angle_45 = -std::f32::consts::FRAC_PI_4;
        let now = time.elapsed_secs();

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    ..default()
                },
                Hitmark::new(now),
                GlobalZIndex(999),
                DespawnAfter(now + hit_detection.despawn_time),
            ))
            .with_children(|parent| {
                spawn_arm(parent, -offset, -offset, angle_45);
                spawn_arm(parent, offset, -offset, neg_angle_45);
                spawn_arm(parent, -offset, offset, neg_angle_45);
                spawn_arm(parent, offset, offset, angle_45);
            });
    }
}

pub fn update_size(
    mut hitmark_query: Query<(&Hitmark, &Children)>,
    mut node_query: Query<(&mut Node, &mut UiTransform)>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();

    for (hitmark, children) in hitmark_query.iter_mut() {
        let age = now - hitmark.spawn_time;
        if age > 0.1 {
            continue;
        }

        let t = age / 0.1;
        let current_width = hitmark.width + 10.0 * t;
        let current_gap = hitmark.gap + 5.0 * t;
        let arm_len = current_width / 2.0 - current_gap;
        let offset = current_gap + arm_len / 2.0;
        let thickness = 2.0_f32;

        let offsets = [
            (-offset, -offset),
            (offset, -offset),
            (-offset, offset),
            (offset, offset),
        ];

        for (child, &(dx, dy)) in children.iter().zip(offsets.iter()) {
            if let Ok((mut node, mut transform)) = node_query.get_mut(child) {
                node.left = Val::Px(-arm_len / 2.0);
                node.top = Val::Px(-thickness / 2.0);
                node.width = Val::Px(arm_len);
                transform.translation = Val2::new(Val::Px(dx), Val::Px(dy));
            }
        }
    }
}
