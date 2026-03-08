use bevy::{animation::RepeatAnimation, prelude::*};

use crate::{animations::utils::find_animation_player, weapons::components::weapon::Weapon};

#[derive(Component)]
pub struct AnimationPlayerLinked;

pub fn link_animations(
    mut commands: Commands,
    unlinked: Query<(Entity, &ChildOf), (With<AnimationPlayer>, Without<AnimationPlayerLinked>)>,
    parents: Query<&ChildOf>,
    weapon_roots: Query<&Weapon>,
) {
    for (anim_entity, parent) in &unlinked {
        if let Some(weapon) = find_root_weapon(parent.parent(), &parents, &weapon_roots) {
            commands.entity(anim_entity).insert((
                AnimationGraphHandle(weapon.animation.graph.clone()),
                AnimationPlayerLinked,
            ));
        }
    }
}

fn find_root_weapon<'a>(
    start: Entity,
    parents: &Query<&ChildOf>,
    weapon_roots: &'a Query<&Weapon>,
) -> Option<&'a Weapon> {
    let mut current = start;
    loop {
        if let Ok(weapon) = weapon_roots.get(current) {
            return Some(weapon);
        }
        match parents.get(current) {
            Ok(parent) => current = parent.parent(),
            Err(_) => return None,
        }
    }
}

pub fn play_weapon_animation<'a>(
    animation: &str,
    weapon: &Weapon,
    children: &Children,
    children_query: &Query<&Children>,
    anim_players: &'a mut Query<&mut AnimationPlayer, With<AnimationPlayerLinked>>,
) {
    let node = match animation {
        "Reloading" => weapon.animation.reloading,
        "Shooting" => weapon.animation.shooting,
        _ => return,
    };

    if let Some(mut anim_player) = find_animation_player(children, &children_query, anim_players) {
        anim_player
            .play(node)
            .set_repeat(RepeatAnimation::Never)
            .set_speed(1.0)
            .replay();
    }
}
