use bevy::prelude::*;

use crate::ui::game_menu::{
    components::{GameState, MenuState, Volume},
    systems::{
        button_system, game_menu_spawn, menu_action, menu_setup, setting_button,
        settings_menu_setup, sound_settings_menu_setup,
    },
};

pub mod components;
pub mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), game_menu_spawn)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            );
    }
}
