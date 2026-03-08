use bevy::ecs::component::Component;

use crate::weapons::components::attachments::AttachmentStats;

#[derive(Component, Debug, PartialEq)]
pub struct Grip {
    pub stats: AttachmentStats,
    pub handling: f32,
}
