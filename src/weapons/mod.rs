pub mod components;
pub mod systems;
mod tests;

use bevy::prelude::*;

use systems::{bullet_movement, spawn_bullets};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullets)
            .add_systems(Update, bullet_movement);
    }
}
