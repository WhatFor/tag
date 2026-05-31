use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
        );
    }
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
