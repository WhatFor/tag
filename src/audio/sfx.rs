use crate::prelude::*;
use bevy::prelude::*;

use bevy::audio::Volume;

#[derive(Event)]
pub struct PlaySfx(pub Handle<AudioSource>);

pub struct AudioSfxPlugin;

impl Plugin for AudioSfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_play_sfx);
    }
}

fn on_play_sfx(trigger: On<PlaySfx>, settings: Res<AudioSettings>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(trigger.0.clone()),
        AudioChannel::Sfx,
        PlaybackSettings {
            volume: Volume::Linear(settings.sfx_volume),
            ..PlaybackSettings::DESPAWN
        },
    ));
}
