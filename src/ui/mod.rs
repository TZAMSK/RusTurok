use bevy::prelude::*;

mod components;
mod system_xp;

use system_xp::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, xp_bar_spawn)
            .add_systems(Update, update_xp_bar)
            .add_systems(Update, update_level_indicator)
            .add_systems(Update, update_xp_required)
            .add_systems(Update, update_given_xp);
    }
}
