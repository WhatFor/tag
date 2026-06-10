use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub const GLOBAL_ANIMATION_SPEED: f32 = 1.0;

#[cfg(feature = "dev")]
pub struct DebugToolsPlugin;

#[cfg(feature = "dev")]
impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
        );
    }
}

#[cfg(feature = "dev")]
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
