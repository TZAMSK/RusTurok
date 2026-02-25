use bevy::math::{Quat, Vec3};

#[derive(Debug)]
pub enum WeaponAnimationStance {
    Grounded,
    Sprinting,
    Sliding,
}

#[derive(Debug)]
pub struct WeaponAnimationState {
    pub rotation: Quat,
    pub translation: Vec3,
    pub stance: WeaponAnimationStance,
}

impl WeaponAnimationState {
    fn define_state_by_stance(stance: WeaponAnimationStance) -> Self {
        match weapon_type {
            WeaponAnimationStance::Sprinting => Self::sprinting(),
            WeaponAnimationStance::Sliding => Self::sliding(),
            WeaponAnimationStance::Grounded => Self::grounded(),
        }
    }

    fn grounded() -> Self {
        Self {
            rotation,
            translation: Vec3::new(0.0, 0.0952, -1.440),
            stance: WeaponAnimationStance::Walking,
        }
    }

    fn sprinting() -> Self {
        Self {
            rotation,
            translation: Vec3::new(0.0, 0.0952, -1.440),
            stance: WeaponAnimationStance::Walking,
        }
    }
}
