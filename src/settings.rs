use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, WindowMode, WindowPlugin},
};
use bevy_fps_counter::FpsCounter;

pub fn settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            cursor_options: CursorOptions {
                visible: false,
                grab_mode: CursorGrabMode::Confined,
                ..default()
            },
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }
}

pub fn fps(keyboard: Res<ButtonInput<KeyCode>>, mut diags_state: ResMut<FpsCounter>) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        if diags_state.is_enabled() {
            diags_state.disable();
        } else {
            diags_state.enable();
        }
    }
}

pub fn exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit_app_event: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit_app_event.send(AppExit::Success);
    }
}
