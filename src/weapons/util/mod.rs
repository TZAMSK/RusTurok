use bevy::{camera::visibility::RenderLayers, prelude::*, scene::SceneInstanceReady};

pub fn apply_render_layers_to_children(
    trigger: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    transforms: Query<&Transform>,
    query: Query<(Entity, &RenderLayers)>,
) {
    let Ok((parent, render_layers)) = query.get(trigger.entity) else {
        return;
    };
    children.iter_descendants(parent).for_each(|entity| {
        if transforms.contains(entity) {
            commands.entity(entity).insert(render_layers.clone());
        }
    });

    commands.entity(trigger.observer()).despawn();
}

pub fn debug_render_layers(
    mesh_query: Query<(Entity, Option<&RenderLayers>, Option<&Name>), With<Mesh3d>>,
) {
    for (entity, render_layers, name) in mesh_query.iter() {
        let layer_info = match render_layers {
            Some(layers) => format!("{:?}", layers),
            None => "No RenderLayers (defaults to layer 0)".to_string(),
        };
        let name_info = name
            .map(|n| n.as_str().to_string())
            .unwrap_or_else(|| format!("Entity {:?}", entity));
        println!("{}: {}", name_info, layer_info);
    }
}
