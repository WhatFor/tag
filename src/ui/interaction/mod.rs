use bevy::prelude::*;

use crate::ui::interaction::image_tint::ImageTintInteractionPlugin;

pub mod image_tint;

pub struct UIInteractionPlugin;

impl Plugin for UIInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ImageTintInteractionPlugin);
    }
}
