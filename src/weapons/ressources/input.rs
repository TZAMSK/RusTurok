use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WeaponInput {
    pub ads_pressed: bool,
    pub should_cancel_sprint: bool,
    pub shoot_pressed: bool,
    pub shoot_blocked_until_release: bool,
    pub should_point_weapon_slide: bool,
    pub ads_blocked: bool,
}

pub fn handle_weapon_input(
    mut input: ResMut<WeaponInput>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let was_ads = input.ads_pressed;
    input.ads_pressed = mouse_button.pressed(MouseButton::Right);
    input.should_cancel_sprint = !was_ads && input.ads_pressed;

    if !input.ads_pressed {
        input.ads_blocked = false;
    }

    if mouse_button.just_released(MouseButton::Left) {
        input.shoot_blocked_until_release = false;
    }

    let was_shooting = input.shoot_pressed;
    let raw_shoot = mouse_button.pressed(MouseButton::Left);

    input.shoot_pressed = raw_shoot && !input.shoot_blocked_until_release;
    input.should_point_weapon_slide = !was_shooting && input.shoot_pressed;
}
