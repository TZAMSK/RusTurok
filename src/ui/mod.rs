use bevy::prelude::*;

mod components;
pub mod game_menu;
mod system_bullet;
mod system_xp;

use system_bullet::*;
use system_xp::*;

use crate::ui::game_menu::{components::GameState, MenuPlugin};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(MenuPlugin)
            .add_systems(Startup, xp_bar_spawn)
            .add_systems(Startup, bullet_indicator_spawn)
            .add_systems(Update, update_xp_bar)
            .add_systems(Update, update_weapon_info_indicator);
    }
}
