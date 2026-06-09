use bevy::prelude::*;

use crate::sets::PlayingSet;
use crate::state::GameState;
use crate::ui::FontAssets;
use crate::ui::events::NarrationComplete;
use crate::ui::widgets::animation::text_fall::{AnimateTextFall, AnimateTextFallComplete};
use crate::world::components::{Area, AreaNarration};
use crate::world::events::PlayerEnteredArea;

#[derive(Component)]
struct ContainerNode;

pub struct NarrationUIPlugin;

impl Plugin for NarrationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            init.in_set(PlayingSet::InitialiseUI),
        );

        app.add_observer(on_player_enter_area);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        ContainerNode,
        Name::new("Narration Container"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        DespawnOnExit(GameState::Playing),
    ));
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_narration: Query<&AreaNarration, With<Area>>,
    container: Single<Entity, With<ContainerNode>>,
    fonts: Res<FontAssets>,
) {
    // Clean up old content
    commands.entity(*container).despawn_related::<Children>();

    // If we have no narration for the Area, skip.
    let Ok(narration) = all_area_narration.get(event.0) else {
        return;
    };

    if narration.lines.len() == 0 {
        return;
    }

    commands
        .spawn((
            AnimateTextFall {
                lines: narration.lines.clone(),
                font: fonts.narration_font.clone(),
                color: fonts.narration_color,
            },
            ChildOf(*container),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .observe(|_: On<AnimateTextFallComplete>, mut commands: Commands| {
            commands.trigger(NarrationComplete);
        });
}
