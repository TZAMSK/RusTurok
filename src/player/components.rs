use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { speed: 20.0 }
    }
}
