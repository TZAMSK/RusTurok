use bevy::{
    prelude::*,
    window::{CursorOptions, WindowMode, WindowPlugin},
};

pub fn settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            cursor_options: CursorOptions {
                visible: false,
                ..default()
            },
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }
}

pub fn exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit_app_event: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit_app_event.send(AppExit::Success);
    }
}
