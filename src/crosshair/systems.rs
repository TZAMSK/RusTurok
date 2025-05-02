use bevy::{
    color::palettes::css::SEA_GREEN, prelude::*, render::view::RenderLayers, window::PrimaryWindow,
};

use crate::camera::renderlayers::CROSSHAIR_LENDER_LAYER;

use super::components::Crosshair;

pub fn spawn_crosshair(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let crosshair = Crosshair::new();

    commands.spawn((
        Node {
            width: Val::Px(crosshair.size),
            height: Val::Px(crosshair.size),
            position_type: PositionType::Absolute,
            ..default()
        },
        Mesh2d(meshes.add(Circle::new(crosshair.size / 2.0))),
        MeshMaterial2d(material.add(ColorMaterial::from_color(SEA_GREEN))),
        RenderLayers::layer(CROSSHAIR_LENDER_LAYER),
        Transform::from_xyz(window.width() / 2.0, window.width() / 2.0, 1.0),
        Camera2d,
    ));
}
