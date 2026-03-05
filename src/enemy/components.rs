use bevy::prelude::Component;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub xp: f32,
    pub level: u32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            health: 50.0,
            xp: 10.0,
            level: 1,
        }
    }
}
