use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Shop {
    pub unlocked: bool,
}

impl Shop {
    pub fn new() -> Self {
        Self { unlocked: false }
    }

    pub fn unlock(&mut self) {
        self.unlocked = true;
    }
}
