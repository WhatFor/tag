use crate::prelude::*;
use bevy::prelude::*;

use crate::audio::interaction::AudioInteractionPlugin;
use crate::audio::sfx::AudioSfxPlugin;
use crate::audio::soundtrack::SoundtrackPlugin;

use bevy::audio::Volume;

pub mod components;
pub mod interaction;
pub mod sfx;
pub mod soundtrack;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioInteractionPlugin);
        app.add_plugins(SoundtrackPlugin);
        app.add_plugins(AudioSfxPlugin);

        app.add_systems(Update, apply_channel_volume);
    }
}

fn apply_channel_volume(
    settings: Res<AudioSettings>,
    mut sinks: Query<(&mut AudioSink, &AudioChannel)>,
) {
    if !settings.is_changed() {
        return;
    }

    for (mut sink, channel) in &mut sinks {
        let v = match channel {
            AudioChannel::Ambience => settings.ambience_volume,
            AudioChannel::Sfx => settings.sfx_volume,
            AudioChannel::Music => settings.music_volume,
        };

        sink.set_volume(Volume::Linear(v * settings.master_volume));
    }
}
