mod weapon;

use bevy::{audio::Volume, prelude::*};

use crate::audio::weapon::play_weapon_audio;

#[derive(Resource)]
pub struct SFXShotVolume(pub Entity);

#[derive(Resource)]
pub struct SFXHitmarkVolume(pub Entity);

#[derive(Resource)]
pub struct MusicVolume;

pub struct GameVolumePlugin;

impl Plugin for GameVolumePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_weapon_audio)
            .add_systems(PostUpdate, change_volume);
    }
}

fn change_volume(
    sfx_shot_sound: Option<Res<SFXShotVolume>>,
    sfx_hitmark_sound: Option<Res<SFXHitmarkVolume>>,
    mut sinks: Query<&mut AudioSink>,
) {
    if let Some(sfx_shot) = sfx_shot_sound {
        if let Ok(mut sink) = sinks.get_mut(sfx_shot.0) {
            sink.set_volume(Volume::Linear(0.1));
        }
    }

    if let Some(sfx_hitmark) = sfx_hitmark_sound {
        if let Ok(mut sink) = sinks.get_mut(sfx_hitmark.0) {
            sink.set_volume(Volume::Linear(0.1 * 0.73));
        }
    }
}
