use bevy::asset::Handle;

use crate::weapons::components::attachments::{grip::Grip, mag::Mag, muzzle::Muzzle, optic::Optic};

pub mod grip;
pub mod mag;
pub mod muzzle;
pub mod optic;

#[derive(Debug, PartialEq)]
pub struct Attachment {
    pub optic: Option<Optic>,
    pub mag: Mag,
    pub grip: Option<Grip>,
    pub muzzle: Option<Muzzle>,
}

#[derive(Debug, PartialEq)]
pub struct AttachmentStats {
    pub name: String,
    pub rarity: Rarity,
}

#[derive(Debug, PartialEq)]
pub enum Rarity {
    Standard,
    Rare,
    Legendary,
    Mythic,
    Relic,
}

impl Default for Attachment {
    fn default() -> Self {
        Self {
            optic: None,
            mag: Mag {
                stats: AttachmentStats::default(),
                asset: Handle::default(),
                bullets: 20,
            },
            grip: None,
            muzzle: None,
        }
    }
}

impl Attachment {
    pub fn new(optic: Option<Optic>, mag: Mag, grip: Option<Grip>, muzzle: Option<Muzzle>) -> Self {
        Self {
            optic,
            mag,
            grip,
            muzzle,
        }
    }
}

impl Default for AttachmentStats {
    fn default() -> Self {
        Self {
            name: String::from(""),
            rarity: Rarity::Standard,
        }
    }
}
