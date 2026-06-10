use bevy::prelude::*;

use crate::ui::widgets::animation::AnimationWidgetsPlugin;
use crate::ui::widgets::button::ButtonWidgetPlugin;
use crate::ui::widgets::tooltip::TooltipWidgetPlugin;

pub mod animation;
pub mod button;
pub mod tooltip;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnimationWidgetsPlugin);
        app.add_plugins(ButtonWidgetPlugin);
        app.add_plugins(TooltipWidgetPlugin);
    }
}
