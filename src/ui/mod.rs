use bevy::prelude::*;

mod components;
mod game_menu;
mod system_bullet;
mod system_xp;

use game_menu::systems::game_menu_spawn;
use system_bullet::*;
use system_xp::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, xp_bar_spawn)
            .add_systems(Startup, bullet_indicator_spawn)
            .add_systems(Update, update_xp_bar)
            .add_systems(Startup, game_menu_spawn)
            .add_systems(Update, update_weapon_info_indicator);
    }
}
