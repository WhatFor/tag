use crate::prelude::*;
use bevy::prelude::*;

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

fn init(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        AreaNameText,
        Name::new("Area Name Text"),
        Text::new(""),
        fonts.title_font.clone(),
        fonts.title_color,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(3.0),
            left: Val::Percent(3.0),
            ..Default::default()
        },
        DespawnOnExit(GameState::Playing),
    ));
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    all_areas: Query<&crate::components::DisplayName, With<Area>>,
    mut area_name_text: Single<&mut Text, With<AreaNameText>>,
) {
    let Ok(current_area_name) = all_areas.get(event.0) else {
        return;
    };

    **area_name_text = Text::new(current_area_name.0.clone());
}
