mod camera;
mod crosshair;
mod player;
mod settings;
mod weapons;
mod world;

use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin,
};

use camera::CameraPlugin;
use crosshair::CrosshairPlugin;
use player::PlayerPlugin;
use settings::{exit, fps, settings};
use weapons::WeaponPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(settings()))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(fps())
        .add_plugins(CrosshairPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WeaponPlugin)
        .add_systems(Update, exit)
        .run();
}
