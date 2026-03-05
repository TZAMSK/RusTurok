use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WeaponInput {
    pub ads_pressed: bool,
    pub should_cancel_sprint: bool,
    pub shoot_pressed: bool,
    pub should_point_weapon_slide: bool,
}

pub fn handle_weapon_input(
    mut input: ResMut<WeaponInput>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let was_ads = input.ads_pressed;
    input.ads_pressed = mouse_button.pressed(MouseButton::Right);
    input.should_cancel_sprint = !was_ads && input.ads_pressed;

    let was_shooting = input.shoot_pressed;
    input.shoot_pressed = mouse_button.pressed(MouseButton::Left);
    input.should_point_weapon_slide = !was_shooting && input.shoot_pressed;
}
