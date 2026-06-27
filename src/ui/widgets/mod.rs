use bevy::prelude::*;

use crate::ui::interaction::UIInteractionPlugin;
use crate::ui::widgets::animation::AnimationWidgetsPlugin;
use crate::ui::widgets::button::ButtonWidgetPlugin;
use crate::ui::widgets::checkbox::CheckboxWidgetPlugin;
use crate::ui::widgets::panel::PanelWidgetPlugin;
use crate::ui::widgets::scroll_area::ScrollAreaWidgetPlugin;
use crate::ui::widgets::tooltip::TooltipWidgetPlugin;

pub mod animation;
pub mod button;
pub mod checkbox;
pub mod panel;
pub mod scroll_area;
pub mod tooltip;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnimationWidgetsPlugin);
        app.add_plugins(ButtonWidgetPlugin);
        app.add_plugins(CheckboxWidgetPlugin);
        app.add_plugins(TooltipWidgetPlugin);
        app.add_plugins(PanelWidgetPlugin);
        app.add_plugins(ScrollAreaWidgetPlugin);
        app.add_plugins(UIInteractionPlugin);
    }
}
