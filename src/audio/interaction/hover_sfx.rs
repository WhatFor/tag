use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HoverSfx(Handle<AudioSource>);

impl HoverSfx {
    pub fn from(sfx: Handle<AudioSource>) -> Self {
        Self(sfx)
    }
}

pub struct HoverSfxInteractionPlugin;

impl Plugin for HoverSfxInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_hover);
    }
}

fn on_hover(trigger: On<Pointer<Over>>, query: Query<&HoverSfx>, mut commands: Commands) {
    if let Ok(sfx) = query.get(trigger.event_target()) {
        commands.trigger(PlaySfx(sfx.0.clone()));
    }
}
