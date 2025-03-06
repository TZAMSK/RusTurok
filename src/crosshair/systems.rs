use bevy::{prelude::*, window::PrimaryWindow};

use super::components::Crosshair;

pub fn spawn_crosshair(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let crosshair = Crosshair::new();

    commands.spawn(ImageBundle {
        node: Node {
            width: Val::Px(crosshair.size),
            height: Val::Px(crosshair.size),
            position_type: PositionType::Absolute,
            left: Val::Px(window.width() / 2.0 - crosshair.size / 2.0),
            bottom: Val::Px(window.height() / 2.0 - crosshair.size / 2.0),
            ..default()
        },
        background_color: BackgroundColor(Color::srgba(0.14, 0.4, 0.2, 1.0)),
        border_radius: BorderRadius::MAX,
        ..default()
    });
}
