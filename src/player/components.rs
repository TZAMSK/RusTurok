use bevy::{math::Vec3, prelude::Component};

#[derive(Debug, Component)]
pub struct Player {
    pub level_info: Level,
    pub movement: Movement,
}

#[derive(Debug, Component)]
pub struct Level {
    pub xp: f32,
    pub level: u32,
    pub xp_required_next_level: f32,
    pub given_xp: f32,
    pub xp_timer: f32,
}

#[derive(Debug, Component)]
pub struct Movement {
    pub speed: f32,
    pub jump_height: f32,
    pub is_grounded: bool,
    pub velocity: f32,
    pub is_sprinting: bool,
    pub is_sliding: bool,
    pub is_crouching: bool,
    pub slide_time: f32,
    pub slide_direction: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            level_info: Level {
                xp: 0.0,
                level: 1,
                xp_required_next_level: 100.0,
                given_xp: 0.0,
                xp_timer: 2.0,
            },
            movement: Movement {
                speed: 7.0,
                jump_height: 7.0,
                is_grounded: true,
                velocity: 0.0,
                is_sprinting: false,
                is_sliding: false,
                is_crouching: false,
                slide_time: 0.7,
                slide_direction: Vec3::new(0.0, 0.0, 0.0),
            },
        }
    }

    pub fn add_xp(&mut self, amount: f32) {
        self.level_info.xp += amount;
        self.level_info.given_xp += amount;
        self.level_info.xp_timer = 1.5;

        if self.level_info.xp >= self.level_info.xp_required_next_level {
            self.level_info.xp = 0.0;
            self.level_info.level += 1;
            self.level_info.xp_required_next_level = (self.level_info.level as f32) * 100.0 * 1.2;
        }
    }

    pub fn xp_progress(&self) -> f32 {
        self.level_info.xp / self.level_info.xp_required_next_level
    }
}
