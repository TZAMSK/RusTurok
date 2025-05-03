use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};

use crate::camera::renderlayers::VIEW_MODEL_RENDER_LAYER;

use super::components::{PrimaryWeaponType, Weapon, WeaponType};

pub fn spawn_weapon(
    parent: &mut impl ChildBuild,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    parent.spawn((
        Mesh3d(arm),
        MeshMaterial3d(arm_material),
        Transform::from_xyz(0.2, -0.1, -0.25),
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        NotShadowCaster,
        Weapon::new(
            "a gun".to_string(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
        ),
    ));
}

pub fn spawn_bullets(mut commands: Commands) {
    commands.spawn(());
}
