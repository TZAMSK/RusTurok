use bevy::prelude::*;

#[derive(Component)]
pub struct GunAnimation {
    pub wobble: GunWobble,
    pub bob: GunBob,
}

impl Default for GunAnimation {
    fn default() -> Self {
        Self {
            wobble: GunWobble::default(),
            bob: GunBob::default(),
        }
    }
}

#[derive(Component)]
pub struct GunWobble {
    pub base_offset: Vec3,
    pub current_offset: Vec3,
    pub intensity: f32,
    pub smoothness: f32,
    pub time: f32,
}

impl Default for GunWobble {
    fn default() -> Self {
        Self {
            base_offset: Vec3::new(0.0, 0.05, 0.7),
            current_offset: Vec3::ZERO,
            intensity: 0.02,
            smoothness: 0.1,
            time: 0.0,
        }
    }
}

#[derive(Component)]
pub struct GunBob {
    pub bob_intensity: f32,
    pub bob_speed: f32,
    pub phase: f32,
}

impl Default for GunBob {
    fn default() -> Self {
        Self {
            bob_intensity: 0.01,
            bob_speed: 10.0,
            phase: 0.0,
        }
    }
}

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

        let effective_previous_rotation = if self.animation_progress < 1.0 {
            let t = ease_out_cubic(self.animation_progress);
            let from = self.previous_coords.1;
            let to = self.rotation;
            from.lerp(to, t)
        } else {
            current_rotation
        };

        let effective_previous_translation = if self.animation_progress < 1.0 {
            let t = ease_out_cubic(self.animation_progress);
            self.previous_coords.0.lerp(self.translation, t)
        } else {
            current_translation
        };

        self.rotation = new_state.rotation;
        self.translation = new_state.translation;
        self.stance = new_state.stance;
        self.animation_transition_speed = new_state.animation_transition_speed;
        self.previous_coords = (effective_previous_translation, effective_previous_rotation);
        self.animation_progress = 0.0;
    }

    fn sprinting() -> Self {
        Self {
            rotation: Vec3::new(-0.55, 1.25, 0.0),
            translation: Vec3::new(0.4, -0.19, -0.25),
            stance: WeaponAnimationStance::Sprinting,
            previous_coords: (Vec3::ZERO, Vec3::ZERO),
            animation_progress: 1.0,
            animation_transition_speed: 3.6,
        }
    }

    fn sliding() -> Self {
        Self {
            rotation: Vec3::new(0.6, 0.0, 0.0),
            translation: Vec3::new(0.16, -0.17, -0.21),
            stance: WeaponAnimationStance::Sliding,
            previous_coords: (Vec3::ZERO, Vec3::ZERO),
            animation_progress: 1.0,
            animation_transition_speed: 3.6,
        }
    }

    fn grounded() -> Self {
        Self {
            rotation: Vec3::new(0.0, 0.0, 0.0),
            translation: Vec3::new(0.26, -0.35, -0.14),
            stance: WeaponAnimationStance::Grounded,
            previous_coords: (Vec3::ZERO, Vec3::ZERO),
            animation_progress: 1.0,
            animation_transition_speed: 3.6,
        }
    }
}

pub fn ease_out_cubic(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(3)
}
