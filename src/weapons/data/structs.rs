use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeaponFile {
    pub weapons: Vec<WeaponData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MagsFile {
    pub mags: Vec<MagData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpticsFile {
    pub optics: Vec<OpticData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuzzlesFile {
    pub muzzles: Vec<MuzzleData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GripsFile {
    pub grips: Vec<GripData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeaponData {
    pub id: String,
    pub name: String,
    pub sound: String,
    pub weapon_type: String,
    pub total_bullets: u32,
    pub stats: StatsData,
    pub assets: AssetsData,
    pub attachments: AttachmentsData,
    pub ads_position: Vec3,
}

impl WeaponData {
    pub fn attach_mag(&mut self, mag: MagData) {
        self.total_bullets += mag.bullets;
        self.stats.reload += mag.bonus_reload;
        self.attachments.mag = Some(mag);
    }

    pub fn attach_optic(&mut self, optic: OpticData) {
        self.stats.zoom += optic.bonus_zoom;
        self.ads_position += optic.delta_ads_position;
        self.attachments.optic = Some(optic);
    }

    pub fn attach_muzzle(&mut self, muzzle: MuzzleData) {
        self.stats.stability += muzzle.bonus_stability;
        self.attachments.muzzle = Some(muzzle);
    }

    pub fn attach_grip(&mut self, grip: GripData) {
        self.stats.handling += grip.bonus_handling;
        self.attachments.grip = Some(grip);
    }

    pub fn remove_mag(&mut self) {
        if let Some(mag) = self.attachments.mag.take() {
            self.total_bullets = self.total_bullets.saturating_sub(mag.bullets);
            self.stats.reload -= mag.bonus_reload;
        }
    }

    pub fn remove_optic(&mut self) {
        if let Some(optic) = self.attachments.optic.take() {
            self.stats.zoom -= optic.bonus_zoom;
            self.ads_position -= optic.delta_ads_position;
        }
    }

    pub fn remove_muzzle(&mut self) {
        if let Some(muzzle) = self.attachments.muzzle.take() {
            self.stats.stability -= muzzle.bonus_stability;
        }
    }

    pub fn remove_grip(&mut self) {
        if let Some(grip) = self.attachments.grip.take() {
            self.stats.handling -= grip.bonus_handling;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetsData {
    pub model: String,
    pub reload_sound: String,
    pub shooting_sound: String,
    pub reload: String,
    pub shooting: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsData {
    pub range: f32,
    pub stability: f32,
    pub handling: f32,
    pub reload: f32,
    pub rounds_per_minute: f32,
    pub aim_assist: f32,
    pub zoom: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentsData {
    pub mag: Option<MagData>,
    pub optic: Option<OpticData>,
    pub muzzle: Option<MuzzleData>,
    pub grip: Option<GripData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MagData {
    pub id: String,
    pub name: String,
    pub bullets: u32,
    pub bonus_reload: f32,
    pub asset: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpticData {
    pub id: String,
    pub name: String,
    pub bonus_zoom: f32,
    pub delta_ads_position: Vec3,
    pub asset: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MuzzleData {
    pub id: String,
    pub name: String,
    pub bonus_stability: f32,
    pub sound: String,
    pub asset: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GripData {
    pub id: String,
    pub name: String,
    pub bonus_handling: f32,
}
