use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ClickSfx(Handle<AudioSource>);

impl ClickSfx {
    pub fn from(sfx: Handle<AudioSource>) -> Self {
        Self(sfx)
    }
}

pub struct ClickSfxInteractionPlugin;

impl Plugin for ClickSfxInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_click);
    }
}

fn on_click(trigger: On<Pointer<Click>>, query: Query<&ClickSfx>, mut commands: Commands) {
    if let Ok(sfx) = query.get(trigger.event_target()) {
        commands.trigger(PlaySfx::new(sfx.0.clone()));
    }
}
