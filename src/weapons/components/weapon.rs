use bevy::prelude::*;

use crate::weapons::{
    components::attachments::Attachment,
    data::structs::{StatsData, WeaponData},
    recoil::{apply_stability, auto_rifle_patterns::ak47_spray_pattern, components::Recoil},
};

#[derive(Debug, Component, PartialEq)]
pub struct Weapon {
    pub id: String,
    pub name: String,
    pub unique_trait: WeaponTrait,
    pub fire_cooldown: f32,
    pub animation: WeaponAnimations,
    pub attachments: Attachment,
}

#[derive(Debug, PartialEq)]
pub struct WeaponTrait {
    pub mag_size: u32,
    pub current_magazine_bullets: u32,
    pub current_reserve_bullets: u32,
    pub stats: Stats,
    pub total_bullets: u32,
    pub weapon_type: WeaponType,
    pub recoil: Recoil,
}

#[derive(Debug, PartialEq)]
pub struct WeaponAnimations {
    pub graph: Handle<AnimationGraph>,
    pub shooting: AnimationNodeIndex,
    pub reloading: AnimationNodeIndex,
}

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub range: f32,
    pub stability: f32,
    pub handling: f32,
    pub reload: f32,
    pub rounds_per_minute: f32,
    pub aim_assist: f32,
    pub zoom: f32,
    pub level: u32,
    pub kills: u32,
    pub kills_next_level: u32,
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

impl Weapon {
    pub fn new(
        id: String,
        name: String,
        weapon_type: WeaponType,
        graph: Handle<AnimationGraph>,
        shooting: AnimationNodeIndex,
        reloading: AnimationNodeIndex,
        attachments: Attachment,
        weapon_data: &WeaponData,
    ) -> Self {
        Self {
            id,
            name,
            unique_trait: WeaponTrait::from_data(weapon_type, weapon_data),
            fire_cooldown: 0.0,
            animation: WeaponAnimations {
                graph,
                shooting,
                reloading,
            },
            attachments,
        }
    }

    pub fn cone_fogiveness(&self) -> (f32, f32) {
        let aim_assist = self.unique_trait.stats.aim_assist;
        let cone = (aim_assist * 0.02).to_radians();
        let bend = (aim_assist / 100.0) * 0.2;
        (cone, bend)
    }
}

impl WeaponTrait {
    pub fn from_data(weapon_type: WeaponType, data: &WeaponData) -> WeaponTrait {
        let stats = Stats::from_data(&data.stats);
        let stability = stats.stability;

        let mag_size = data
            .attachments
            .mag
            .as_ref()
            .map(|m| m.bullets)
            .unwrap_or(data.total_bullets);

        Self {
            mag_size,
            current_magazine_bullets: mag_size,
            current_reserve_bullets: data.total_bullets,
            total_bullets: data.total_bullets,
            recoil: Recoil {
                base_pattern: ak47_spray_pattern(),
                pattern: apply_stability(&ak47_spray_pattern(), stability),
                recoil_reset_time: 0.5,
                current_bullet_index: 0,
                time_since_last_shot: 0.8,
            },
            stats,
            weapon_type,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            range: 40.0,
            stability: 40.0,
            handling: 40.0,
            reload: 30.0,
            rounds_per_minute: 20.0 / 210.0,
            aim_assist: 20.0,
            zoom: 14.0,
            level: 1,
            kills: 0,
            kills_next_level: 10,
        }
    }
}

impl Stats {
    pub fn add_kill(&mut self) {
        self.kills += 1;

        if self.kills >= self.kills_next_level {
            self.kills = 0;
            self.level += 1;
            self.kills_next_level = ((self.kills_next_level as f32) * 1.2) as u32;
        }
    }

    pub fn level_progress(&self) -> f32 {
        self.kills as f32 / self.kills_next_level as f32
    }

    pub fn from_data(data: &StatsData) -> Self {
        Self {
            range: data.range,
            stability: data.stability,
            handling: data.handling,
            reload: data.reload,
            rounds_per_minute: data.rounds_per_minute,
            aim_assist: data.aim_assist,
            zoom: data.zoom,
            ..default()
        }
    }
}

impl Default for WeaponTrait {
    fn default() -> Self {
        Self {
            mag_size: 20,
            current_magazine_bullets: 20,
            current_reserve_bullets: 200,
            stats: Stats::default(),
            total_bullets: 200,
            weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::Sidearm),
            recoil: Recoil {
                base_pattern: ak47_spray_pattern(),
                pattern: ak47_spray_pattern(),
                current_bullet_index: 1,
                recoil_reset_time: 0.5,
                time_since_last_shot: 0.0,
            },
        }
    }
}

pub fn weapon_type_from_str(s: &str) -> WeaponType {
    match s {
        "AutoRifle" => WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
        "HandCannon" => WeaponType::PrimaryWeaponType(PrimaryWeaponType::HandCannon),
        "Sidearm" => WeaponType::PrimaryWeaponType(PrimaryWeaponType::Sidearm),
        "Shotgun" => WeaponType::SecondaryWeaponType(SecondaryWeaponType::Shotgun),
        "Sniper" => WeaponType::SecondaryWeaponType(SecondaryWeaponType::Sniper),
        _ => {
            warn!("Unknown weapon_type '{}', defaulting to Sidearm", s);
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::Sidearm)
        }
    }
}
