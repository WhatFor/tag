use bevy::prelude::*;
use bevy::ui::FlexWrap::NoWrap;

use crate::assets::character_loader::CharacterStore;
use crate::global::PausableSystems;
use crate::state::GameState;
use crate::ui::FontAssets;
use crate::ui::events::DialogueComplete;
use crate::world::components::{Area, AreaDialogue};
use crate::world::events::PlayerEnteredArea;

#[derive(Component)]
struct ContainerNode;

#[derive(Component)]
struct DialogueWrapperNode;

#[derive(Component)]
struct LineNode;

#[derive(Component)]
struct LastInChain;

#[derive(Component)]
struct NextChar {
    pub entity: Option<Entity>,
    pub delay: Option<f32>,
}

#[derive(Component)]
struct AnimationDelay(pub Timer);

#[derive(Component)]
struct AnimationProgress(pub f32);

#[derive(Component)]
struct AnimationComplete;

pub struct DialogueUIPlugin;

impl Plugin for DialogueUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init);

        app.add_systems(
            Update,
            animate
                .run_if(in_state(GameState::Playing))
                .in_set(PausableSystems),
        );

        app.add_systems(
            Update,
            tick_delay
                .run_if(in_state(GameState::Playing))
                .in_set(PausableSystems),
        );

        app.add_systems(
            Update,
            on_char_done
                .run_if(in_state(GameState::Playing))
                .in_set(PausableSystems),
        );

        app.add_observer(on_player_enter_area);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        ContainerNode,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));
}

const CHAR_ENTER_SPEED: f32 = 15.0;
const CHAR_Y_OFFSET: f32 = 30.0;
const NEXT_CHAR_THRESHOLD: f32 = 0.3;
const LINE_DELAY_SECS: f32 = 0.6;

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_dialogue: Query<&AreaDialogue, With<Area>>,
    character_store: Res<CharacterStore>,
    container: Single<Entity, With<ContainerNode>>,
    fonts: Res<FontAssets>,
) {
    // Clean up old content
    commands.entity(*container).despawn_related::<Children>();

    // If we have no dialogue for the Area, skip.
    let Ok(dialogue) = all_area_dialogue.get(event.0) else {
        return;
    };

    if dialogue.lines.len() == 0 {
        return;
    }

    let mut line_nodes: Vec<Entity> = vec![];

    let mut first_char_animated = false;
    let mut last_char_of_prev_line: Option<Entity> = None;

    // Spawn a wrapper to hold the dialogue lines to make it easy to position other elements
    let dialogue_wrapper = commands
        .spawn((
            DialogueWrapperNode,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    for line in dialogue.lines.iter() {
        let line_node = commands
            .spawn((
                LineNode,
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .id();

        // Pass 1: spawn in forward order
        let char_entities: Vec<Entity> = line
            .chars()
            .map(|ch| {
                commands
                    .spawn((
                        Text::new(ch.to_string()),
                        fonts.dialogue_font.clone(),
                        TextColor(fonts.dialogue_color.0.with_alpha(0.0)),
                        Node {
                            position_type: PositionType::Relative,
                            bottom: Val::Px(CHAR_Y_OFFSET),
                            ..default()
                        },
                    ))
                    .id()
            })
            .collect();

        // Pass 2: link each entity to the next
        for i in 0..char_entities.len() {
            let next = char_entities.get(i + 1).copied();
            commands.entity(char_entities[i]).insert(NextChar {
                entity: next,
                delay: None,
            });
        }

        // Link lines together
        if let (Some(prev_last), Some(&current_first)) =
            (last_char_of_prev_line, char_entities.first())
        {
            commands.entity(prev_last).insert(NextChar {
                entity: Some(current_first),
                delay: Some(LINE_DELAY_SECS),
            });
        }

        // First char of the whole sequence starts animating
        if !first_char_animated {
            if let Some(&first) = char_entities.first() {
                commands.entity(first).insert(AnimationProgress(0.0));
                first_char_animated = true;
            }
        }

        commands.entity(line_node).add_children(&char_entities);
        line_nodes.push(line_node);

        // Last char needs a marker
        if line_nodes.len() == dialogue.lines.len() {
            let last_char = char_entities.last().expect("Unable to read last char.");
            commands.entity(*last_char).insert(LastInChain);
        }

        last_char_of_prev_line = char_entities.last().copied();
    }

    // Draw all the lines onto the wrapper
    commands.entity(dialogue_wrapper).add_children(&line_nodes);

    // Draw the speaker name
    let speaker = character_store
        .0
        .get(&dialogue.character_id)
        .expect("Character not found!");

    let speaker_name_text = commands
        .spawn((
            Text::new("- ".to_owned() + &speaker.display_name.clone()),
            TextLayout {
                linebreak: LineBreak::NoWrap,
                ..default()
            },
            fonts.dialogue_font.clone(),
            fonts.dialogue_color,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(-50.0),
                right: Val::Px(-150.0),
                ..default()
            },
        ))
        .id();

    commands
        .entity(dialogue_wrapper)
        .add_child(speaker_name_text);

    // Finally, draw everything
    commands
        .entity(*container)
        .replace_children(&[dialogue_wrapper]);
}

fn animate(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut AnimationProgress,
        &mut TextColor,
        &mut Node,
        Option<&mut NextChar>,
    )>,
) {
    for (entity, mut progress, mut color, mut node, next_char) in &mut query {
        progress.0 = (progress.0 + time.delta_secs() * CHAR_ENTER_SPEED).min(1.0);

        color.0.set_alpha(progress.0);
        node.bottom = Val::Px(CHAR_Y_OFFSET * (1.0 - progress.0));

        if let Some(mut next) = next_char {
            if progress.0 >= NEXT_CHAR_THRESHOLD {
                // Take the next_entity so not to repeat
                if let Some(next_entity) = next.entity.take() {
                    // If a delay is present, apply AnimationDelay
                    if let Some(delay) = next.delay {
                        commands
                            .entity(next_entity)
                            .insert(AnimationDelay(Timer::from_seconds(delay, TimerMode::Once)));
                    // Else apply the animation directly.
                    } else {
                        commands.entity(next_entity).insert(AnimationProgress(0.0));
                    }
                }
            }
        }

        if progress.0 >= 1.0 {
            commands
                .entity(entity)
                .remove::<AnimationProgress>()
                .insert(AnimationComplete);
        }
    }
}

fn tick_delay(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationDelay)>,
    mut commands: Commands,
) {
    for (entity, mut pending) in &mut query {
        if pending.0.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .remove::<AnimationDelay>()
                .insert(AnimationProgress(0.0));
        }
    }
}

fn on_char_done(
    mut commands: Commands,
    query: Query<Entity, (Added<AnimationComplete>, With<LastInChain>)>,
) {
    for _ in query {
        info!("Dialogue animation completed!");
        commands.trigger(DialogueComplete);
    }
}
