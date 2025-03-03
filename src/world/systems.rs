use bevy::{prelude::*, render::view::RenderLayers};

use crate::camera::renderlayers::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 100.0,
            range: 10.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, -4.0, -0.8),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}
