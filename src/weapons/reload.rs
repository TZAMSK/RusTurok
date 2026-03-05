use bevy::{audio::PlaybackMode, prelude::*};

use crate::weapons::components::Weapon;

pub fn reload_weapon(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<&mut Weapon>,
    time: Res<Time>,
) {
    let Ok(mut weapon) = weapon_query.single_mut() else {
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
        }
    }
}
