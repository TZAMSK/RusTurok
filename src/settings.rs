use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PresentMode, WindowMode, WindowPlugin},
};

use crate::ui::game_menu::components::{GameState, MenuState};

pub fn settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(bevy::window::Window {
            present_mode: PresentMode::AutoNoVsync,
            mode: WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary),
            ..default()
        }),
        primary_cursor_options: Some(CursorOptions {
            visible: false,
            grab_mode: CursorGrabMode::Confined,
            ..default()
        }),
        ..default()
    }
}

/*
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
*/

pub fn open_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Main);
    }
}
