use bevy::prelude::*;

mod components;
mod system_bullet;
mod system_dmg;
mod system_xp;

use system_bullet::*;
use system_dmg::*;
use system_xp::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, xp_bar_spawn)
            .add_systems(Startup, bullet_indicator_spawn)
            .add_systems(Update, dmg_indicator_spawn)
            .add_systems(Update, update_xp_bar)
            .add_systems(Update, update_weapon_info_indicator)
            .add_systems(Update, update_dmg_indicator);
    }
}
