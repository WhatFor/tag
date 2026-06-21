use bevy::prelude::*;

use crate::audio::interaction::AudioInteractionPlugin;
use crate::audio::soundtrack::SoundtrackPlugin;

pub mod ext;
pub mod interaction;
pub mod soundtrack;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioInteractionPlugin);
        app.add_plugins(SoundtrackPlugin);
    }
}
