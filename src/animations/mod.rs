pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::animations::systems::link_animations;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            link_animations.before(crate::weapons::reload::reload_weapon),
        );
    }
}
