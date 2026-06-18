use crate::prelude::*;
use bevy::prelude::*;

use crate::game::events::PlayerGameOver;

// TODO: remove me
use crate::ui::content::NarrationContainerNode;

pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ExploringState::AwaitingContinue),
            show_continue_prompt.in_set(PausableSystems),
        );

        app.add_systems(
            OnEnter(ExploringState::AwaitingChoice),
            show_choice_prompt.in_set(PausableSystems),
        );

        app.add_systems(
            Update,
            wait_for_continue_input
                .run_if(in_state(ExploringState::AwaitingContinue))
                .in_set(PausableSystems),
        );

        app.add_systems(
            OnEnter(ExploringState::AwaitingGameOver),
            show_game_over.in_set(PausableSystems),
        );
    }
}

fn show_continue_prompt(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(ExploringState::AwaitingContinue),
            GlobalZIndex(LAYER_HUD),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(250.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(button("Continue")).observe(
                |_: On<Pointer<Click>>,
                 mut commands: Commands,
                 current_dialogue: Single<Entity, With<NarrationContainerNode>>| {
                    commands.trigger(PlayerContinued);
                    commands.entity(current_dialogue.entity()).try_despawn();
                },
            );
        });
}

fn wait_for_continue_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(PlayerContinued);
    }
}

fn show_choice_prompt(
    mut commands: Commands,
    current_area: Single<&mut CurrentArea, With<Player>>,
    areas: Query<&AreaExits, With<Area>>,
) {
    let Ok(current_area_exits) = areas.get(current_area.0) else {
        return;
    };

    commands
        .spawn((
            DespawnOnExit(ExploringState::AwaitingChoice),
            GlobalZIndex(LAYER_HUD),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(250.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|container| {
                for exit in &current_area_exits.0 {
                    match exit {
                        AreaExit::Choice(area_exit_options) => {
                            for exit_option in area_exit_options {
                                let label = exit_option.label.clone();
                                let to = exit_option.to.clone();

                                container.spawn(button(label)).observe(
                                    move |_: On<Pointer<Click>>,
                                          mut commands: Commands,
                                          current_dialogue: Single<
                                        Entity,
                                        With<NarrationContainerNode>,
                                    >| {
                                        commands.trigger(PlayerChose(to.clone()));
                                        commands.entity(current_dialogue.entity()).try_despawn();
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
        });
}

fn show_game_over(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::GameOver),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(250.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
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
                p.spawn(Text::new("Game over..."));
                p.spawn(button("Main menu")).observe(
                    |_: On<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(PlayerGameOver);
                    },
                );
            });
        });
}
