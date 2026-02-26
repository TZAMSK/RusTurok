use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponAnimationStance {
    Grounded,
    Sprinting,
    Sliding,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct WeaponAnimationState {
    pub rotation: Vec3,
    pub translation: Vec3,
    pub stance: WeaponAnimationStance,
    pub previous_coords: (Vec3, Vec3),
    pub animation_progress: f32, // 0.0 to 1.0
    pub animation_transition_speed: f32,
}

impl WeaponAnimationState {
    pub fn define_state_by_stance(stance: WeaponAnimationStance) -> Self {
        match stance {
            WeaponAnimationStance::Sprinting => Self::sprinting(),
            WeaponAnimationStance::Sliding => Self::sliding(),
            WeaponAnimationStance::Grounded => Self::grounded(),
        }
    }

    pub fn change_state_by_stance(
        &mut self,
        stance: WeaponAnimationStance,
        current_translation: Vec3,
        current_rotation: Vec3,
    ) {
        if self.stance == stance && self.animation_progress >= 1.0 {
            return;
        }

        let new_state = Self::define_state_by_stance(stance);

        self.rotation = new_state.rotation;
        self.translation = new_state.translation;
        self.stance = new_state.stance;
        self.animation_transition_speed = new_state.animation_transition_speed;
        self.previous_coords = (current_translation, current_rotation);
        self.animation_progress = 0.0;
    }

    fn sprinting() -> Self {
        Self {
            rotation: Vec3::new(-0.55, 1.25, 0.0),
            translation: Vec3::new(0.4, -0.19, -0.25),
            stance: WeaponAnimationStance::Sprinting,
            previous_coords: (Vec3::ZERO, Vec3::ZERO),
            animation_progress: 1.0,
            animation_transition_speed: 1.6,
        }
    }

    fn sliding() -> Self {
        Self {
            rotation: Vec3::new(1.15, 0.0, 0.0),
            translation: Vec3::new(0.16, -0.17, -0.21),
            stance: WeaponAnimationStance::Sliding,
            previous_coords: (Vec3::ZERO, Vec3::ZERO),
            animation_progress: 1.0,
            animation_transition_speed: 1.6,
        }
    }

    fn grounded() -> Self {
        Self {
            rotation: Vec3::new(0.0, 0.0, 0.0),
            translation: Vec3::new(0.26, -0.35, 0.0),
            stance: WeaponAnimationStance::Grounded,
            previous_coords: (Vec3::ZERO, Vec3::ZERO),
            animation_progress: 1.0,
            animation_transition_speed: 1.6,
        }
    }
}

pub fn apply_transition_animation(
    mut weapon_query: Query<(&mut WeaponAnimationState, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut weap_state, mut transform) in weapon_query.iter_mut() {
        if weap_state.animation_progress >= 1.0 {
            continue;
        }

        weap_state.animation_progress += weap_state.animation_transition_speed * time.delta_secs();
        weap_state.animation_progress = weap_state.animation_progress.clamp(0.0, 1.0);

        let t = ease_out_cubic(weap_state.animation_progress);

        let target_position = weap_state.previous_coords.0.lerp(weap_state.translation, t);
        transform.translation = target_position;

        let start_rotation = Quat::from_euler(
            EulerRot::YXZ,
            weap_state.previous_coords.1.y,
            weap_state.previous_coords.1.x,
            weap_state.previous_coords.1.z,
        );

        let target_rotation = Quat::from_euler(
            EulerRot::YXZ,
            weap_state.rotation.y,
            weap_state.rotation.x,
            weap_state.rotation.z,
        );

        transform.rotation = start_rotation.slerp(target_rotation, t);
    }
}

fn ease_out_cubic(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(3)
}
