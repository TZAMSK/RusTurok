use crate::{
    ui::components::{BulletCount, KillsBarFill, WeaponLVLIndicator},
    weapons::components::weapon::Weapon,
};
use bevy::prelude::*;

pub fn bullet_indicator_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(20.0),
            height: Val::Percent(10.0),
            right: Val::Percent(5.0),
            bottom: Val::Percent(5.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(70.0),
                        border: UiRect::px(0.0, 0.0, 3.0, 3.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                ))
                .with_children(|sub_parent| {
                    sub_parent
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Percent(0.0),
                                left: Val::Percent(0.0),
                                ..default()
                            },
                            Text::new(""),
                            TextFont {
                                font_size: 13.0,
                                ..Default::default()
                            },
                        ))
                        .insert(WeaponLVLIndicator);

                    sub_parent
                        .spawn(Node {
                            flex_grow: 1.0,
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            overflow: Overflow::clip(),
                            ..default()
                        })
                        .with_children(|img_parent| {
                            img_parent.spawn((
                                Node {
                                    height: Val::Percent(100.0),
                                    width: Val::Auto,
                                    ..default()
                                },
                                ImageNode::new(asset_server.load("placeholder/ak.png")),
                            ));
                        });

                    sub_parent
                        .spawn((
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Percent(100.0),
                                border: UiRect {
                                    left: Val::Px(1.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexEnd,
                                padding: UiRect {
                                    right: Val::Px(20.0),
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                        ))
                        .with_children(|text_parent| {
                            text_parent
                                .spawn((
                                    Text::new(""),
                                    TextFont {
                                        font_size: 28.0,
                                        ..Default::default()
                                    },
                                ))
                                .insert(BulletCount);
                        });
                });

            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(7.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.02)),
                ))
                .with_children(|child| {
                    child
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(7.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(
                                227.0 / 255.0,
                                27.0 / 255.0,
                                107.0 / 255.0,
                                0.4,
                            )),
                        ))
                        .insert(KillsBarFill);
                });
        });
}

pub fn update_weapon_info_indicator(
    weapon_query: Query<&Weapon>,
    mut bar_query: Query<&mut Node, With<KillsBarFill>>,
    mut bullet_count_indicator_query: Query<&mut Text, With<BulletCount>>,
    mut weapon_level_indicator_query: Query<
        &mut Text,
        (With<WeaponLVLIndicator>, Without<BulletCount>),
    >,
) {
    let Ok(weapon) = weapon_query.single() else {
        return;
    };

    if let Ok(mut bullet_count_indicator) = bullet_count_indicator_query.single_mut() {
        **bullet_count_indicator = format!(
            "{}/{}",
            weapon.unique_trait.current_magazine_bullets,
            weapon.unique_trait.current_reserve_bullets,
        );
    }

    if let Ok(mut weapon_level_indicator) = weapon_level_indicator_query.single_mut() {
        **weapon_level_indicator = format!("LVL {}", weapon.unique_trait.stats.level);
    }

    if let Ok(mut style) = bar_query.single_mut() {
        style.width = Val::Percent(weapon.unique_trait.stats.level_progress() * 100.0);
    }
}
