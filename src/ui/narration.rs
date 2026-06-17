use crate::prelude::*;
use crate::ui::NarrationContainerNode;
use bevy::prelude::*;

use crate::ui::layout::GameArea;
use crate::ui::widgets::animation::text_fall::AnimateTextFall;
use crate::ui::widgets::animation::text_fall::AnimateTextFallComplete;

pub struct NarrationUIPlugin;

impl Plugin for NarrationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_enter_area);
    }
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_narration: Query<&AreaNarration, With<Area>>,
    fonts: Res<FontAssets>,
    game_area: Single<Entity, With<GameArea>>,
) {
    // If we have no narration for the Area, skip.
    let Ok(narration) = all_area_narration.get(event.0) else {
        return;
    };

    if narration.lines.is_empty() {
        return;
    }

    let container = commands
        .spawn((
            ChildOf(game_area.entity()),
            DespawnOnExit(GameState::Playing),
            NarrationContainerNode,
            GlobalZIndex(LAYER_GAME),
            Name::new("Narration Container"),
            Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            AnimateTextFall {
                lines: narration.lines.clone(),
                font: fonts.narration_font.clone(),
                color: fonts.narration_color,
            },
            ChildOf(container),
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
