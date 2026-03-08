use bevy::{asset::Handle, ecs::component::Component, scene::Scene};

use crate::weapons::components::attachments::AttachmentStats;

#[derive(Component, Debug, PartialEq)]
pub struct Mag {
    pub stats: AttachmentStats,
    pub asset: Handle<Scene>,
    pub bullets: u32,
}
