pub mod database;
pub mod structs;

use bevy::prelude::*;

use crate::weapons::data::database::WeaponDatabase;

pub struct WeaponDatabasePlugin;

impl Plugin for WeaponDatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeaponDatabase::load());
    }
}
