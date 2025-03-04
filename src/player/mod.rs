pub mod components;
mod systems;

use bevy::prelude::*;

use systems::move_player_camera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player_camera);
    }
}
