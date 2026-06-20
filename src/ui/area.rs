use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::layout::HudAreaBottomLeft;

#[derive(Component)]
struct AreaNameText;

pub struct AreaUIPlugin;

impl Plugin for AreaUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            init.in_set(PlayingSet::InitialiseUI),
        );

        app.add_observer(on_player_enter_area);
    }
}

fn init(
    mut commands: Commands,
    hud_area: Single<Entity, With<HudAreaBottomLeft>>,
    fonts: Res<FontAssets>,
) {
    commands.spawn((
        AreaNameText,
        ChildOf(hud_area.entity()),
        GlobalZIndex(LAYER_HUD),
        Name::new("Area Name Text"),
        Text::new(""),
        fonts.title_font.clone(),
        fonts.title_color,
        Node {
            ..Default::default()
        },
        DespawnOnExit(GameState::Playing),
    ));
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    all_areas: Query<&DisplayName, With<Area>>,
    mut area_name_text: Single<&mut Text, With<AreaNameText>>,
) {
    let Ok(current_area_name) = all_areas.get(**event) else {
        return;
    };

    **area_name_text = Text::new((*current_area_name).clone());
}
