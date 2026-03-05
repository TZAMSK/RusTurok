mod camera;
mod combat;
mod crosshair;
pub mod enemy;
mod player;
mod settings;
mod ui;
mod weapons;
mod world;

use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

use bevy::text::TextPlugin;
use camera::CameraPlugin;
use crosshair::CrosshairPlugin;
use player::PlayerPlugin;
use settings::{exit_game, settings};
use weapons::animation::GunAnimationState;
use weapons::{BulletPlugin, WeaponPlugin};
use world::WorldPlugin;

use crate::enemy::EnemyPlugin;
use crate::ui::UIPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(settings()))
        //.add_plugins(fps())
        .add_plugins(CrosshairPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(UIPlugin)
        .add_systems(Update, exit_game)
        .init_resource::<GunAnimationState>();

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/Font.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}
