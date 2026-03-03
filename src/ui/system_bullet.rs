use crate::{ui::components::BulletCount, weapons::components::Weapon};
use bevy::prelude::*;

pub fn bullet_indicator_spawn(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(20.0),
                height: Val::Percent(7.0),
                right: Val::Percent(5.0),
                bottom: Val::Percent(5.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Percent(5.0),
                        top: Val::Percent(20.0),
                        bottom: Val::Percent(20.0),
                        ..default()
                    },
                    Text::new(""),
                    TextFont {
                        font_size: 40.0,
                        ..Default::default()
                    },
                ))
                .insert(BulletCount);
        });
}

pub fn update_bullet_indicator(
    weapon_query: Query<&Weapon>,
    mut bullet_count_indicator_query: Query<&mut Text, With<BulletCount>>,
) {
    if let Ok(weapon) = weapon_query.single() {
        if let Ok(mut bullet_count_indicator) = bullet_count_indicator_query.single_mut() {
            **bullet_count_indicator = format!(
                "{}/{}",
                weapon.unique_trait.current_magazine_bullets,
                weapon.unique_trait.current_reserve_bullets.to_string(),
            );
        }
    }
}
