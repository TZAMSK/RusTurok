use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, WindowMode, WindowPlugin},
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub fn settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync,
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

pub fn fps() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 10.0,
                ..default()
            },
            ..default()
        },
    }
}

pub fn exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit_app_event: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit_app_event.write(AppExit::Success);
    }
}
