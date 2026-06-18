use crate::prelude::*;
use crate::ui::layout::HudAreaTop;
use bevy::prelude::*;

use crate::ui::layout::GameArea;
use crate::ui::widgets::animation::text_fall::AnimateTextFall;
use crate::ui::widgets::animation::text_fall::AnimateTextFallComplete;

// TODO: Remove me?
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NarrationContainerNode;

pub struct ContentUIPlugin;

impl Plugin for ContentUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_enter_area);
    }
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_content: Query<&AreaContent, With<Area>>,
    character_store: Res<CharacterStore>,
    fonts: Res<FontAssets>,
    game_area: Single<Entity, With<GameArea>>,
    hud_area_top: Single<Entity, With<HudAreaTop>>,
) {
    let Ok(content) = all_area_content.get(event.0) else {
        return;
    };

    match content {
        AreaContent::Dialogue {
            character_id,
            lines,
        } => {
            if lines.is_empty() {
                return;
            }

            // Draw the speaker name
            let speaker = character_store
                .0
                .get(&character_id.clone().unwrap_or(String::from("")))
                .expect("Character not found!");

            commands.spawn((
                Name::new("Dialogue Speaker Text Container"),
                ChildOf(hud_area_top.entity()),
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    Text::new(speaker.display_name.clone()),
                    Name::new("Dialogue Speaker Text"),
                    TextLayout {
                        linebreak: LineBreak::NoWrap,
                        ..default()
                    },
                    fonts.ui_font.clone(),
                    fonts.ui_color,
                )],
            ));

            let container = commands
                .spawn((
                    ChildOf(game_area.entity()),
                    DespawnOnExit(GameState::Playing),
                    NarrationContainerNode,
                    GlobalZIndex(LAYER_GAME),
                    Name::new("Dialogue Container"),
                    Node {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .id();

            // Draw dialogue text
            let dialogue_wrapper = commands
                .spawn((
                    ChildOf(container.entity()),
                    Name::new("Dialogue Wrapper"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    DespawnOnExit(GameState::Playing),
                ))
                .id();

            commands
                .spawn((
                    AnimateTextFall {
                        lines: lines.clone(),
                        font: fonts.dialogue_font.clone(),
                        color: fonts.dialogue_color,
                    },
                    ChildOf(dialogue_wrapper),
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
                    commands.trigger(DialogueComplete);
                });
        }
        AreaContent::Narration { lines } => {
            if lines.is_empty() {
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
                        lines: lines.clone(),
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
    }
}
