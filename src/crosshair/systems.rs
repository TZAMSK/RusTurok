use bevy::{color::palettes::css::SEA_GREEN, prelude::*};

use super::components::Crosshair;

pub fn spawn_crosshair(mut commands: Commands) {
    let crosshair = Crosshair::new();

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(crosshair.size),
                height: Val::Px(crosshair.size),
                margin: UiRect::axes(
                    Val::Px(-crosshair.size / 2.0),
                    Val::Px(-crosshair.size / 2.0),
                ),
                ..default()
            },
            BackgroundColor(SEA_GREEN.into()),
            GlobalZIndex(999),
        ))
        .with_child((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(SEA_GREEN.into()),
        ));
}
