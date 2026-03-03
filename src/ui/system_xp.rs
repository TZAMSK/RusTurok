use super::components::XPBarFill;
use crate::{
    player::components::Player,
    ui::components::{GivenXPIndicator, LevelIndicator, XPRequiredIndicator},
};
use bevy::prelude::*;

pub fn xp_bar_spawn(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(40.0),
            height: Val::Percent(2.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(30.0),
            bottom: Val::Percent(1.0),
            ..default()
        })
        .with_children(|parent| {
            //Level indicator
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Percent(100.0),
                        left: Val::Percent(0.0),
                        ..default()
                    },
                    Text::new(""),
                    TextFont {
                        font_size: 40.0,
                        ..Default::default()
                    },
                ))
                .insert(LevelIndicator);

            //XP indicator
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Percent(100.0),
                        right: Val::Percent(0.0),
                        ..default()
                    },
                    Text::new(""),
                    TextFont {
                        font_size: 15.0,
                        ..Default::default()
                    },
                ))
                .insert(XPRequiredIndicator);

            //Progress bar
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.02)),
                ))
                .with_children(|child| {
                    child
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundGradient(vec![Gradient::Linear(LinearGradient {
                                color_space: InterpolationColorSpace::Srgba,
                                angle: 90.0,
                                stops: vec![
                                    ColorStop {
                                        color: Color::srgba(
                                            255.0 / 255.0,
                                            135.0 / 255.0,
                                            66.0 / 255.0,
                                            0.2,
                                        ),
                                        ..default()
                                    },
                                    ColorStop {
                                        color: Color::srgba(
                                            255.0 / 255.0,
                                            233.0 / 255.0,
                                            133.0 / 255.0,
                                            0.2,
                                        ),
                                        ..default()
                                    },
                                ],
                            })]),
                        ))
                        .insert(XPBarFill);

                    child
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                right: Val::Percent(1.0),
                                top: Val::Percent(1.0),
                                ..default()
                            },
                            Text::new(""),
                            TextFont {
                                font_size: 9.0,
                                ..Default::default()
                            },
                        ))
                        .insert(GivenXPIndicator);
                });
        });
}

pub fn update_xp_bar(
    mut player_query: Query<&mut Player>,
    mut bar_query: Query<&mut Node, With<XPBarFill>>,
) {
    if let Ok(player) = player_query.single_mut() {
        if let Ok(mut style) = bar_query.single_mut() {
            let progress = player.xp_progress() * 100.0;
            style.width = Val::Percent(progress);
        }
    }
}

pub fn update_level_indicator(
    player_query: Query<&Player>,
    mut level_indicator_query: Query<&mut Text, With<LevelIndicator>>,
) {
    if let Ok(player) = player_query.single() {
        if let Ok(mut level_indicator) = level_indicator_query.single_mut() {
            **level_indicator = player.level_info.level.to_string();
        }
    }
}

pub fn update_xp_required(
    player_query: Query<&Player>,
    mut xp_required_indicator_query: Query<&mut Text, With<XPRequiredIndicator>>,
) {
    if let Ok(player) = player_query.single() {
        if let Ok(mut xp_required_indicator) = xp_required_indicator_query.single_mut() {
            **xp_required_indicator = format!(
                "{}/{}",
                player.level_info.xp.to_string(),
                player.level_info.xp_required_next_level.to_string(),
            );
        }
    }
}

pub fn update_given_xp(
    mut player_query: Query<&mut Player>,
    mut given_xp_indicator_query: Query<&mut Text, With<GivenXPIndicator>>,
    time: Res<Time>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        if let Ok(mut given_xp_indicator) = given_xp_indicator_query.single_mut() {
            if player.level_info.xp_timer > 0.0 {
                player.level_info.xp_timer -= time.delta_secs();
                **given_xp_indicator = format!("+{}", player.level_info.given_xp.to_string());
            }

            if player.level_info.xp_timer < 0.0 || player.level_info.given_xp == 0.0 {
                player.level_info.given_xp = 0.0;
                **given_xp_indicator = format!("");
            }
        }
    }
}
