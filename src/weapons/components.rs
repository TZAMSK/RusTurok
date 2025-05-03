use bevy::prelude::Component;

#[derive(Debug, Component, PartialEq)]
pub struct Weapon {
    pub name: String,
    pub unique_trait: WeaponTrait,
}

#[derive(Debug, PartialEq)]
pub struct WeaponTrait {
    pub bullet_speed: f32,
    pub mag_size: u32,
    pub stats: Stats,
    pub total_bullets: u32,
    pub weapon_type: WeaponType,
}

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub range: f32,
    pub stability: f32,
    pub handling: f32,
    pub reload: f32,
    pub round_per_minute: f32,
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
            round_per_minute: 210.0,
            aim_assist: 10.0,
            zoom: 14.0,
        }
    }
}
impl Default for WeaponTrait {
    fn default() -> Self {
        Self {
            bullet_speed: 100.0,
            mag_size: 20,
            stats: Stats::default(),
            total_bullets: 200,
            weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::Sidearm),
        }
    }
}

impl Weapon {
    pub fn new(name: String, weapon_type: WeaponType) -> Self {
        Self {
            name,
            unique_trait: WeaponTrait::define_stats_by_type(weapon_type),
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
            mag_size: 32,
            stats: Stats {
                range: 20.0,
                stability: 50.0,
                handling: 50.0,
                reload: 45.0,
                round_per_minute: 600.0,
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
