use crate::prelude::*;
use bevy::prelude::*;

use bevy::audio::Volume;

#[derive(Event)]
pub struct PlaySfx {
    pub source: Handle<AudioSource>,
    pub speed: f32,
}

impl PlaySfx {
    pub fn new(source: Handle<AudioSource>) -> Self {
        Self { source, speed: 1.0 }
    }

    pub fn with_speed(source: Handle<AudioSource>, speed: f32) -> Self {
        Self { source, speed }
    }
}

pub struct AudioSfxPlugin;

impl Plugin for AudioSfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_play_sfx);
    }
}

fn on_play_sfx(trigger: On<PlaySfx>, settings: Res<AudioSettings>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(trigger.source.clone()),
        AudioChannel::Sfx,
        PlaybackSettings {
            volume: Volume::Linear(settings.sfx_volume * settings.master_volume),
            speed: trigger.speed,
            ..PlaybackSettings::DESPAWN
        },
    ));
}
