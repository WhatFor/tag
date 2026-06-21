use bevy::prelude::*;

use crate::audio::interaction::AudioInteractionPlugin;

pub mod ext;
pub mod interaction;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioInteractionPlugin);
    }
}
