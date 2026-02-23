use bevy::prelude::Component;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Enemy {
    pub fn new() -> Self {
        Self { health: 1.0 }
    }
}
