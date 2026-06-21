use bevy::prelude::*;

use crate::audio::interaction::click_sfx::ClickSfxInteractionPlugin;
use crate::audio::interaction::hover_sfx::HoverSfxInteractionPlugin;

pub mod click_sfx;
pub mod hover_sfx;

pub struct AudioInteractionPlugin;

impl Plugin for AudioInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HoverSfxInteractionPlugin);
        app.add_plugins(ClickSfxInteractionPlugin);
    }
}
