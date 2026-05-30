use bevy::prelude::*;

use crate::state::GameState;
use crate::ui::FontAssets;
use crate::world::components::Area;
use crate::world::events::PlayerEnteredArea;

#[derive(Component)]
struct AreaNameText;

pub struct AreaUIPlugin;

impl Plugin for AreaUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn);
        app.add_observer(update);
    }
}

fn spawn(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        AreaNameText,
        Text::new(""),
        fonts.title_font.clone(),
        fonts.title_color,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(3.0),
            left: Val::Percent(3.0),
            ..Default::default()
        },
    ));
}

fn update(
    event: On<PlayerEnteredArea>,
    all_areas: Query<&crate::components::DisplayName, With<Area>>,
    mut area_name_text: Single<&mut Text, With<AreaNameText>>,
) {
    let Ok(current_area_name) = all_areas.get(event.0) else {
        return;
    };

    **area_name_text = Text::new(current_area_name.0.clone());
}
