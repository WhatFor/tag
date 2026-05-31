use bevy::prelude::*;

use crate::ui::widgets::button::ButtonWidgetPlugin;

pub mod button;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ButtonWidgetPlugin);
    }
}
