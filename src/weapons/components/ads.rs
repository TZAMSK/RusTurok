use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Default)]
pub struct ADS {
    pub is_ads: bool,
    pub ads_progress: f32, // 0.0 = hip fire, 1.0 = fully ads
    pub ads_speed: f32,
    pub hip_position: Vec3,
    pub ads_position: Vec3,
    pub hip_fov: f32,
    pub ads_fov: f32,
}

impl ADS {
    pub fn new(hip_position: Vec3, ads_position: Vec3) -> Self {
        Self {
            is_ads: false,
            ads_progress: 0.0,
            ads_speed: 8.0,
            hip_position,
            ads_position,
            hip_fov: 120.0,
            ads_fov: 80.0,
        }
    }
}
