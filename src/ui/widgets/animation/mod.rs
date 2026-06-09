use bevy::prelude::*;

use crate::ui::widgets::animation::text_fall::TextFallAnimationPlugin;

pub mod text_fall;

pub struct AnimationWidgetsPlugin;

impl Plugin for AnimationWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextFallAnimationPlugin);
    }
}
