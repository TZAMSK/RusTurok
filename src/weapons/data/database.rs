use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::weapons::data::structs::{
    GripData, GripsFile, MagData, MagsFile, MuzzleData, MuzzlesFile, OpticData, OpticsFile,
    WeaponData, WeaponFile,
};

#[derive(Resource, Default)]
pub struct WeaponDatabase {
    pub weapons: HashMap<String, WeaponData>,
    pub grips: HashMap<String, GripData>,
    pub mags: HashMap<String, MagData>,
    pub muzzles: HashMap<String, MuzzleData>,
    pub optics: HashMap<String, OpticData>,
}

impl WeaponDatabase {
    pub fn load() -> Self {
        let weapons_json = include_str!("../../../assets/data/weapons.json");
        let grips_json = include_str!("../../../assets/data/attachments/grips.json");
        let mags_json = include_str!("../../../assets/data/attachments/mags.json");
        let muzzles_json = include_str!("../../../assets/data/attachments/muzzles.json");
        let optics_json = include_str!("../../../assets/data/attachments/optics.json");

        let weapons_file: WeaponFile =
            serde_json::from_str(weapons_json).expect("Failed to parse weapons.json");
        let grips_file: GripsFile =
            serde_json::from_str(grips_json).expect("Failed to parse grips.json");
        let mags_file: MagsFile =
            serde_json::from_str(mags_json).expect("Failed to parse mags.json");
        let muzzles_file: MuzzlesFile =
            serde_json::from_str(muzzles_json).expect("Failed to parse muzzles.json");
        let optics_file: OpticsFile =
            serde_json::from_str(optics_json).expect("Failed to parse optics.json");

        let mut db = WeaponDatabase::default();

        for w in weapons_file.weapons {
            db.weapons.insert(w.id.clone(), w);
        }
        for g in grips_file.grips {
            db.grips.insert(g.id.clone(), g);
        }
        for m in mags_file.mags {
            db.mags.insert(m.id.clone(), m);
        }
        for m in muzzles_file.muzzles {
            db.muzzles.insert(m.id.clone(), m);
        }
        for o in optics_file.optics {
            db.optics.insert(o.id.clone(), o);
        }

        db
    }

    pub fn get_weapon(&self, id: &str) -> Option<&WeaponData> {
        self.weapons.get(id)
    }

    pub fn get_grip(&self, id: &str) -> Option<&GripData> {
        self.grips.get(id)
    }

    pub fn get_mag(&self, id: &str) -> Option<&MagData> {
        self.mags.get(id)
    }

    pub fn get_muzzle(&self, id: &str) -> Option<&MuzzleData> {
        self.muzzles.get(id)
    }

    pub fn get_optic(&self, id: &str) -> Option<&OpticData> {
        self.optics.get(id)
    }

    pub fn get_weapon_mut(&mut self, id: &str) -> Option<&mut WeaponData> {
        self.weapons.get_mut(id)
    }
}
