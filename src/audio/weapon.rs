use crate::{
    audio::{SFXHitmarkVolume, SFXShotVolume},
    combat::HitDetection,
    weapons::{bullets::DespawnAfter, components::Weapon, ressources::input::WeaponInput},
};
use bevy::prelude::*;

pub fn play_weapon_audio(
    mut commands: Commands,
    weapon_query: Query<&Weapon>,
    weapon_input: Res<WeaponInput>,
    hit_detection: Res<HitDetection>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let Ok(weapon) = weapon_query.single() else {
        return;
    };

    if weapon_input.shoot_pressed {
        if weapon.unique_trait.recoil.current_bullet_index < weapon.unique_trait.mag_size - 1 {
            let entity = commands
                .spawn((
                    AudioPlayer::new(asset_server.load("sounds/weapons/ak47.ogg")),
                    PlaybackSettings::ONCE,
                    DespawnAfter(time.elapsed_secs() + 0.9),
                ))
                .id();

            commands.insert_resource(SFXShotVolume(entity));
        }
    }

    if hit_detection.hit || hit_detection.killed {
        let entity = commands
            .spawn((
                AudioPlayer::new(asset_server.load("sounds/hitmarker.ogg")),
                PlaybackSettings::ONCE,
                DespawnAfter(time.elapsed_secs() + 0.6),
            ))
            .id();

        commands.insert_resource(SFXHitmarkVolume(entity));
    }
}
