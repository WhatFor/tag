use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Node)]
pub struct AnimateTextFall {
    pub lines: Vec<String>,
    pub font: TextFont,
    pub color: TextColor,
}

#[derive(Component)]
struct NextChar {
    pub entity: Option<Entity>,
    pub delay: Option<f32>,
}

#[derive(Component)]
struct LineNode;

#[derive(Component, Deref, DerefMut)]
struct AnimationDelay(pub Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationProgress(pub f32);

#[derive(Component, Deref)]
struct LastInAnimation(pub Entity);

#[derive(Component)]
struct AnimationComplete;

#[derive(EntityEvent)]
pub struct AnimateTextFallComplete {
    entity: Entity,
}

const CHAR_ENTER_SPEED: f32 = 15. * GLOBAL_ANIMATION_SPEED;
const CHAR_Y_OFFSET: f32 = 30.;
const NEXT_CHAR_THRESHOLD: f32 = 0.3;
const LINE_DELAY_SECS: f32 = 0.6 * GLOBAL_ANIMATION_SPEED;

pub struct TextFallAnimationPlugin;

impl Plugin for TextFallAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_text_fall);

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

        app.add_observer(on_char_appeared);
    }
}

fn on_char_appeared(
    _: On<Add, AnimationProgress>,
    audio: Res<AudioAssets>,
    current_area: Single<&mut CurrentArea, With<Player>>,
    areas: Query<&AreaContent, With<Area>>,
    character_store: Res<CharacterStore>,
    mut commands: Commands,
) {
    let Ok(area_content) = areas.get(current_area.entity()) else {
        return;
    };

    match area_content {
        AreaContent::Dialogue {
            character_id,
            lines: _,
        } => {
            let speaker = character_store
                .get(&character_id.clone().unwrap_or(String::from("")))
                .expect("Character not found!");

            let Some(speech_sfx) = speaker.speech_sfx.clone() else {
                return;
            };

            let Some(sfx) = audio.sfx.get(speech_sfx.as_str()) else {
                warn!("Missing speech sfx '{}'...", speech_sfx);
                return;
            };

            let pitch = random_pitch();
            commands.trigger(PlaySfx::with_speed(sfx.clone(), pitch));
        }
        AreaContent::Narration { .. } => {
            // TODO: No sound effect for narration. For now(?)
        }
        AreaContent::Combat { .. } => {
            // TODO: Does not trigger for combat.
        }
    }
}

fn random_pitch() -> f32 {
    let variance = 0.05;
    let mut buf = [0u8; 4];

    if getrandom::fill(&mut buf).is_err() {
        return 1.0; // fall back to no bend rather than panic
    }

    let unit = u32::from_le_bytes(buf) as f32 / u32::MAX as f32; // [0, 1]

    1.0 + (unit * 2.0 - 1.0) * variance // [1 - variance, 1 + variance]
}

fn build_text_fall(
    mut commands: Commands,
    query: Query<(Entity, &AnimateTextFall), Added<AnimateTextFall>>,
) {
    for (root, animation) in &query {
        let mut line_nodes: Vec<Entity> = vec![];

        let mut first_char_animated = false;
        let mut last_char_of_prev_line: Option<Entity> = None;

        for line in animation.lines.iter() {
            let line_node = commands
                .spawn((
                    LineNode,
                    Name::new("Animation TextFall Line"),
                    Node {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.),
                        row_gap: Val::Px(4.),
                        ..default()
                    },
                ))
                .id();

            let mut char_entities: Vec<Entity> = Vec::new();
            let mut word_nodes: Vec<Entity> = Vec::new();

            // Pass 1: spawn in forward order
            for word in line.split_whitespace() {
                let word_node = commands
                    .spawn((
                        Name::new("Animation TextFall Word"),
                        Node {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                    ))
                    .id();

                let chars_in_word = word
                    .chars()
                    .map(|ch| {
                        commands
                            .spawn((
                                Text::new(ch.to_string()),
                                Name::new("Animation TextFall Line Char"),
                                animation.font.clone(),
                                TextColor(animation.color.with_alpha(0.)),
                                Node {
                                    position_type: PositionType::Relative,
                                    bottom: Val::Px(CHAR_Y_OFFSET),
                                    ..default()
                                },
                            ))
                            .id()
                    })
                    .collect::<Vec<Entity>>();

                commands.entity(word_node).add_children(&chars_in_word);
                word_nodes.push(word_node);
                char_entities.extend(chars_in_word);
            }

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
                    commands.entity(first).insert(AnimationProgress(0.));
                    first_char_animated = true;
                }
            }

            commands.entity(line_node).add_children(&word_nodes);
            line_nodes.push(line_node);

            // Last char needs a marker
            if line_nodes.len() == animation.lines.len() {
                let last_char = char_entities.last().expect("Unable to read last char.");

                commands.entity(*last_char).insert(LastInAnimation(root));
            }

            last_char_of_prev_line = char_entities.last().copied();
        }

        commands.entity(root).add_children(&line_nodes);
    }
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
        **progress = (**progress + time.delta_secs() * CHAR_ENTER_SPEED).min(1.);

        color.0.set_alpha(**progress);
        node.bottom = Val::Px(CHAR_Y_OFFSET * (1. - **progress));

        if let Some(mut next) = next_char {
            if **progress >= NEXT_CHAR_THRESHOLD {
                // Take the next_entity so not to repeat
                if let Some(next_entity) = next.entity.take() {
                    // If a delay is present, apply AnimationDelay
                    if let Some(delay) = next.delay {
                        commands
                            .entity(next_entity)
                            .insert(AnimationDelay(Timer::from_seconds(delay, TimerMode::Once)));
                    // Else apply the animation directly.
                    } else {
                        commands.entity(next_entity).insert(AnimationProgress(0.));
                    }
                }
            }
        }

        if progress.0 >= 1. {
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
        if pending.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .remove::<AnimationDelay>()
                .insert(AnimationProgress(0.));
        }
    }
}

fn on_char_done(mut commands: Commands, query: Query<&LastInAnimation, Added<AnimationComplete>>) {
    for entity in &query {
        commands.trigger(AnimateTextFallComplete { entity: **entity });
    }
}
