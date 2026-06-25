use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::FontHandles;
use crate::ui::layout::GameArea;
use crate::ui::layout::HudAreaBottomCenter;
use crate::ui::layout::HudAreaTop;
use crate::ui::widgets::animation::text_fall::AnimateTextFall;
use crate::ui::widgets::animation::text_fall::AnimateTextFallComplete;

const CHOICE_KEYS: [KeyCode; 4] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
];

#[derive(Event)]
pub struct ContentDisplayCompleted;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AreaContentRoot;

pub struct ContentUIPlugin;

impl Plugin for ContentUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_enter_area);
        app.add_observer(on_content_display_completed);

        app.add_systems(
            OnEnter(ExploringState::AwaitingContentPrompt),
            show_content_prompt.in_set(PausableSystems),
        );

        app.add_systems(
            Update,
            wait_for_keyboard_input
                .run_if(in_state(ExploringState::AwaitingContentPrompt))
                .in_set(PausableSystems),
        );

        app.add_systems(
            OnEnter(ExploringState::AwaitingGameOver),
            show_game_over.in_set(PausableSystems),
        );
    }
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_content: Query<&AreaContent, With<Area>>,
    previous_area_content: Query<Entity, With<AreaContentRoot>>,
    character_store: Res<CharacterStore>,
    fonts: Res<FontAssets>,
    font_handles: Res<FontHandles>,
    game_area: Single<Entity, With<GameArea>>,
    hud_area_top: Single<Entity, With<HudAreaTop>>,
    dyn_content: DynamicContent,
) {
    let Ok(content) = all_area_content.get(**event) else {
        return;
    };

    for entity in &previous_area_content {
        commands.entity(entity).try_despawn();
    }

    match content {
        AreaContent::Dialogue {
            character_id,
            lines,
        } => {
            if lines.is_empty() {
                return;
            }

            let speaker = character_store
                .get(&character_id.clone().unwrap_or(String::from("")))
                .expect("Character not found!");

            let character_colour = speaker.font_colour.unwrap_or(fonts.dialogue_color.0);

            let character_font = match speaker
                .font
                .as_deref()
                .and_then(|name| font_handles.custom.get(name))
            {
                Some(handle) => TextFont {
                    font: handle.clone(),
                    ..fonts.dialogue_font.clone()
                },
                None => fonts.dialogue_font.clone(),
            };

            commands.spawn((
                Name::new("Dialogue Speaker Text Container"),
                AreaContentRoot,
                DespawnOnExit(GameState::Playing),
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
                    character_font.clone(),
                    fonts.ui_color,
                )],
            ));

            let container = commands
                .spawn((
                    ChildOf(game_area.entity()),
                    AreaContentRoot,
                    DespawnOnExit(GameState::Playing),
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

            let dynamic_lines = lines.iter().map(|l| dyn_content.resolve(l)).collect();

            commands
                .spawn((
                    AnimateTextFall {
                        lines: dynamic_lines,
                        font: character_font.clone(),
                        color: TextColor(character_colour),
                    },
                    ChildOf(dialogue_wrapper),
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .observe(|_: On<AnimateTextFallComplete>, mut commands: Commands| {
                    commands.trigger(ContentDisplayCompleted);
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
                    AreaContentRoot,
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

            let dynamic_lines = lines.iter().map(|l| dyn_content.resolve(l)).collect();

            commands
                .spawn((
                    AnimateTextFall {
                        lines: dynamic_lines,
                        font: fonts.narration_font.clone(),
                        color: fonts.narration_color,
                    },
                    ChildOf(container),
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .observe(|_: On<AnimateTextFallComplete>, mut commands: Commands| {
                    commands.trigger(ContentDisplayCompleted);
                });
        }
    }
}

fn on_content_display_completed(
    _: On<ContentDisplayCompleted>,
    current_area: Single<&CurrentArea, With<Player>>,
    areas: Query<(&AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    let Ok((area_id, current_area_exits)) = areas.get(current_area.entity()) else {
        return;
    };

    match current_area_exits.first() {
        Some(AreaExit::Continue(_)) | Some(AreaExit::Choice(_)) => {
            next_exploring_state.set(ExploringState::AwaitingContentPrompt);
        }
        Some(AreaExit::GameOver) => {
            info!("Waiting for GameOver continue {}", **area_id);
            next_exploring_state.set(ExploringState::AwaitingGameOver);
        }
        None => {
            warn!("on_dialogue_completed: area '{}' has no exits", **area_id);
        }
    }
}

fn show_content_prompt(
    mut commands: Commands,
    current_area: Single<&CurrentArea, With<Player>>,
    areas: Query<(&AreaId, &AreaExits), With<Area>>,
    bottom_center_hud: Single<Entity, With<HudAreaBottomCenter>>,
) {
    let Ok((area_id, current_area_exits)) = areas.get(current_area.entity()) else {
        // Area not found - shouldn't happen
        return;
    };

    let area_id = area_id.clone();

    if let Some(AreaExit::Continue(_)) = current_area_exits.first() {
        // Spawn 'Continue' button
        commands
            .spawn((
                Name::new("Continue Button Container"),
                ChildOf(bottom_center_hud.entity()),
                DespawnOnExit(ExploringState::AwaitingContentPrompt),
                GlobalZIndex(LAYER_HUD),
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(button("Continue (space)")).observe(
                    move |_: On<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(PlayerContinued {
                            from: area_id.clone(),
                        });
                    },
                );
            });
    } else {
        // Spawn 'Choice' buttons
        commands
            .spawn((
                Name::new("Choice Buttons Container"),
                ChildOf(bottom_center_hud.entity()),
                DespawnOnExit(ExploringState::AwaitingContentPrompt),
                GlobalZIndex(LAYER_HUD),
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.),
                    ..default()
                },
            ))
            .with_children(|container| {
                for exit in current_area_exits.iter() {
                    match exit {
                        AreaExit::Choice(area_exit_options) => {
                            for (index, exit_option) in area_exit_options.iter().enumerate() {
                                let label = format!("{}. {}", index + 1, exit_option.label);
                                let to = exit_option.to.clone();
                                let chosen_id = exit_option.id.clone();
                                let area_id = area_id.clone();

                                container.spawn(button(label)).observe(
                                    move |_: On<Pointer<Click>>, mut commands: Commands| {
                                        commands.trigger(PlayerChose {
                                            from: area_id.clone(),
                                            to: to.clone(),
                                            chosen_id: chosen_id.clone(),
                                        });
                                    },
                                );
                            }
                        }
                        _ => {
                            warn!("Should not be possible!");
                        }
                    };
                }
            });
    };
}

fn wait_for_keyboard_input(
    mut commands: Commands,
    current_area: Single<&CurrentArea, With<Player>>,
    areas: Query<(&AreaId, &AreaExits), With<Area>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((area_id, current_area_exits)) = areas.get(current_area.entity()) else {
        return;
    };

    match current_area_exits.first() {
        Some(AreaExit::Continue(_)) => {
            if keyboard.just_pressed(KeyCode::Space) {
                commands.trigger(PlayerContinued {
                    from: area_id.clone(),
                });
            }
        }
        Some(AreaExit::Choice(choices)) => {
            for (choice, key_code) in choices.iter().zip(CHOICE_KEYS) {
                if keyboard.just_pressed(key_code) {
                    commands.trigger(PlayerChose {
                        from: area_id.clone(),
                        chosen_id: choice.id.clone(),
                        to: choice.to.clone(),
                    });
                }
            }
        }
        _ => {
            // Ignore where no exits or choice exits
        }
    };
}

fn show_game_over(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::GameOver),
            GlobalZIndex(LAYER_MENU),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor::from(Color::srgb(0., 0., 0.)),
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.),
                ..default()
            })
            .with_children(|p| {
                p.spawn(Text::new("Game over."));
                p.spawn(button("Main menu")).observe(
                    |_: On<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(StopSoundtrack);
                        commands.trigger(PlayerGameOver);
                    },
                );
            });
        });
}
