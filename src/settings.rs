use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PresentMode, PrimaryWindow, WindowMode, WindowPlugin},
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub fn settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(bevy::window::Window {
            present_mode: PresentMode::AutoNoVsync,
            mode: WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary),
            ..default()
        }),
        primary_cursor_options: Some(CursorOptions {
            visible: false,
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

pub fn exit_game(keyboard: Res<ButtonInput<KeyCode>>, mut exit_writer: MessageWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit_writer.write(AppExit::Success);
    }
}
