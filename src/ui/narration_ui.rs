use bevy::prelude::*;

use crate::state::GameState;
use crate::ui::FontAssets;
use crate::ui::events::NarrationComplete;
use crate::world::components::{Area, AreaNarration};
use crate::world::events::PlayerEnteredArea;

#[derive(Component)]
struct ContainerNode;

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

pub struct NarrationUIPlugin;

impl Plugin for NarrationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init);

        app.add_systems(Update, animate.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, tick_delay.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, on_char_done.run_if(in_state(GameState::Playing)));

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

    let mut line_nodes: Vec<Entity> = vec![];

    let mut first_char_animated = false;
    let mut last_char_of_prev_line: Option<Entity> = None;

    for line in narration.lines.iter() {
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
                        fonts.narration_font.clone(),
                        TextColor(fonts.narration_color.0.with_alpha(0.0)),
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
        if line_nodes.len() == narration.lines.len() {
            let last_char = char_entities.last().expect("Unable to read last char.");
            commands.entity(*last_char).insert(LastInChain);
        }

        last_char_of_prev_line = char_entities.last().copied();
    }

    commands.entity(*container).replace_children(&line_nodes);
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
        info!("Narration animation completed!");
        commands.trigger(NarrationComplete);
    }
}
