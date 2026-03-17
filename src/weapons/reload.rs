use bevy::prelude::*;

use crate::{
    animations::systems::{play_weapon_animation, AnimationPlayerLinked},
    weapons::{components::weapon::Weapon, ressources::input::WeaponInput},
};

pub fn reload_weapon(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<(&mut Weapon, &Children)>,
    children_query: Query<&Children>,
    mut anim_players: Query<&mut AnimationPlayer, With<AnimationPlayerLinked>>,
    mut weapon_input: ResMut<WeaponInput>,
) {
    let Ok((mut weapon, children)) = weapon_query.single_mut() else {
        return;
    };

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

            if weapon_input.shoot_pressed {
                weapon_input.shoot_blocked_until_release = true;
            }

            play_weapon_animation(
                "Reloading",
                &weapon,
                children,
                &children_query,
                &mut anim_players,
            );
        }
    }
}
