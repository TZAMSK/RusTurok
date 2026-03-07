use bevy::prelude::*;

use crate::animations::systems::AnimationPlayerLinked;

fn find_animation_player_entity(
    children: &Children,
    children_query: &Query<&Children>,
    anim_players: &Query<&mut AnimationPlayer, With<AnimationPlayerLinked>>,
) -> Option<Entity> {
    for child in children.iter() {
        if anim_players.contains(child) {
            return Some(child);
        }
        if let Ok(grandchildren) = children_query.get(child) {
            if let Some(e) =
                find_animation_player_entity(grandchildren, children_query, anim_players)
            {
                return Some(e);
            }
        }
    }
    None
}

pub fn find_animation_player<'a>(
    children: &Children,
    children_query: &Query<&Children>,
    anim_players: &'a mut Query<&mut AnimationPlayer, With<AnimationPlayerLinked>>,
) -> Option<Mut<'a, AnimationPlayer>> {
    let entity = {
        let immutable: &Query<&mut AnimationPlayer, With<AnimationPlayerLinked>> = anim_players;
        find_animation_player_entity(children, children_query, immutable)
    };
    anim_players.get_mut(entity?).ok()
}
