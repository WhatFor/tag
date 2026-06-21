use crate::prelude::*;
use bevy::prelude::*;

pub trait AudioCommandsExt {
    fn play_sfx(&mut self, sfx: Handle<AudioSource>);
}

impl AudioCommandsExt for Commands<'_, '_> {
    fn play_sfx(&mut self, sfx: Handle<AudioSource>) {
        self.spawn((
            AudioPlayer::new(sfx),
            AudioChannel::Sfx,
            PlaybackSettings::DESPAWN,
        ));
    }
}
