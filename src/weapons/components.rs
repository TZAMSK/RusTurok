use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};

pub use super::wobble::{GunBob, GunWobble};

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

#[derive(Component, Default)]
pub struct ADS {
    pub is_ads: bool,
    pub ads_progress: f32, // 0.0 = hip fire, 1.0 = fully ads
    pub ads_speed: f32,
    pub hip_position: Vec3,
    pub ads_position: Vec3,
    pub hip_fov: f32,
    pub ads_fov: f32,
}

impl ADS {
    pub fn new(hip_position: Vec3, ads_position: Vec3) -> Self {
        Self {
            is_ads: false,
            ads_progress: 0.0,
            ads_speed: 8.0,
            hip_position,
            ads_position,
            hip_fov: 120.0,
            ads_fov: 80.0,
        }
    }
}

#[derive(Component)]
pub struct BulletTracer;

#[derive(Component)]
pub struct BulletDirection(pub Vec3);

#[derive(Component)]
pub struct Bullet;

#[derive(Debug, Component, PartialEq)]
pub struct Weapon {
    pub name: String,
    pub unique_trait: WeaponTrait,
    pub fire_cooldown: f32,
}

#[derive(Debug, PartialEq)]
pub struct WeaponTrait {
    pub bullet_speed: f32,
    pub mag_size: u32,
    pub current_magazine_bullets: u32,
    pub current_reserve_bullets: u32,
    pub stats: Stats,
    pub total_bullets: u32,
    pub weapon_type: WeaponType,
    pub recoil_pattern: Vec<Vec2>,
}

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub range: f32,
    pub stability: f32,
    pub handling: f32,
    pub reload: f32,
    pub seconds_per_shot: f32,
    pub aim_assist: f32,
    pub zoom: f32,
}

#[derive(Debug, PartialEq)]
pub enum WeaponType {
    PrimaryWeaponType(PrimaryWeaponType),
    SecondaryWeaponType(SecondaryWeaponType),
}

#[derive(Debug, PartialEq)]
pub enum PrimaryWeaponType {
    HandCannon,
    AutoRifle,
    Sidearm,
}

#[derive(Debug, PartialEq)]
pub enum SecondaryWeaponType {
    Shotgun,
    Sniper,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            range: 40.0,
            stability: 40.0,
            handling: 40.0,
            reload: 30.0,
            seconds_per_shot: 20.0 / 210.0,
            aim_assist: 10.0,
            zoom: 14.0,
        }
    }
}
impl Default for WeaponTrait {
    fn default() -> Self {
        Self {
            bullet_speed: 1000.0,
            mag_size: 20,
            current_magazine_bullets: 20,
            current_reserve_bullets: 200,
            stats: Stats::default(),
            total_bullets: 200,
            weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::Sidearm),
            recoil_pattern: vec![Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)],
        }
    }
}

impl Weapon {
    pub fn new(name: String, weapon_type: WeaponType) -> Self {
        Self {
            name,
            unique_trait: WeaponTrait::define_stats_by_type(weapon_type),
            fire_cooldown: 0.0,
        }
    }
}

impl WeaponTrait {
    fn define_stats_by_type(weapon_type: WeaponType) -> Self {
        match weapon_type {
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::HandCannon) => Self::hand_cannon(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle) => Self::auto_rifle(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::Sidearm) => Self::sidearm(),
            WeaponType::SecondaryWeaponType(SecondaryWeaponType::Shotgun) => Self::shotgun(),
            WeaponType::SecondaryWeaponType(SecondaryWeaponType::Sniper) => Self::sniper(),
        }
    }

    fn hand_cannon() -> Self {
        Self {
            mag_size: 11,
            total_bullets: 120,
            weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::HandCannon),
            ..Self::default()
        }
    }

    fn auto_rifle() -> Self {
        Self {
            mag_size: 20,
            stats: Stats {
                range: 20.0,
                stability: 50.0,
                handling: 50.0,
                reload: 45.0,
                seconds_per_shot: 60.0 / 600.0,
                aim_assist: 10.0,
                zoom: 14.0,
            },
            total_bullets: 400,
            weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
            ..Self::default()
        }
    }

    fn sidearm() -> Self {
        Self::default()
    }

    fn shotgun() -> Self {
        Self {
            mag_size: 3,
            total_bullets: 20,
            weapon_type: WeaponType::SecondaryWeaponType(SecondaryWeaponType::Shotgun),
            ..Self::default()
        }
    }

    fn sniper() -> Self {
        Self {
            mag_size: 4,
            total_bullets: 20,
            weapon_type: WeaponType::SecondaryWeaponType(SecondaryWeaponType::Sniper),
            ..Self::default()
        }
    }
}

#[derive(Component)]
pub struct BulletImpact {
    pub position: Vec3,
    pub time: f32,
}
