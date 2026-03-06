use bevy::{audio::PlaybackMode, prelude::*};

use crate::weapons::components::Weapon;

#[derive(Resource)]
struct Animations {
    node_indices: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

pub fn reload_weapon(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<&mut Weapon>,
    asset_server: Res<AssetServer>,
    mut players: Query<&mut AnimationPlayer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    time: Res<Time>,
) {
    let Ok(mut weapon) = weapon_query.single_mut() else {
        return;
    };

    let animation_clips =
        [asset_server.load(GltfAssetLabel::Animation(4).from_asset("models/safeak2/ak6.glb"))];
    let mut animation_graph = AnimationGraph::new();

    let node_indices = animation_graph
        .add_clips(animation_clips, 1.0, animation_graph.root)
        .collect();

    commands.insert_resource(Animations {
        node_indices,
        graph: animation_graphs.add(animation_graph),
    });

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if weapon.unique_trait.current_magazine_bullets < weapon.unique_trait.mag_size
            && weapon.unique_trait.current_reserve_bullets > 0
        {
            let needed =
                weapon.unique_trait.mag_size - weapon.unique_trait.current_magazine_bullets;
            let to_reload = needed.min(weapon.unique_trait.current_reserve_bullets);

            weapon.unique_trait.current_reserve_bullets -= to_reload;
            weapon.unique_trait.current_magazine_bullets += to_reload;
            weapon.unique_trait.recoil.current_bullet_index = 0;
        }
    }
}
