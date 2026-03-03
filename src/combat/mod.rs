use bevy::ecs::{entity::Entity, message::Message};

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32,
    pub shooter: Option<Entity>,
}
