mod camera;
mod crosshair;
pub mod enemy;
mod player;
mod settings;
mod weapons;
mod world;

use bevy::prelude::*;

use camera::CameraPlugin;
use crosshair::CrosshairPlugin;
use player::PlayerPlugin;
use settings::{exit_game, settings};
use weapons::animation::GunAnimationState;
use weapons::{BulletPlugin, WeaponPlugin};
use world::WorldPlugin;

use crate::enemy::EnemyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(settings()))
        //.add_plugins(fps())
        .add_plugins(CrosshairPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(BulletPlugin)
        .add_systems(Update, exit_game)
        .init_resource::<GunAnimationState>()
        .run();
}
