use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::animation::text_fall::{AnimateTextFall, AnimateTextFallComplete};

#[derive(Component)]
struct ContainerNode;

#[derive(Component)]
struct DialogueWrapperNode;

pub struct DialogueUIPlugin;

impl Plugin for DialogueUIPlugin {
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
        Name::new("Dialogue Container"),
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

    // Spawn a wrapper to hold the dialogue lines to make it easy to position other elements
    let dialogue_wrapper = commands
        .spawn((
            DialogueWrapperNode,
            Name::new("Dialogue Wrapper"),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(GameState::Playing),
        ))
        .id();

    commands
        .spawn((
            AnimateTextFall {
                lines: dialogue.lines.clone(),
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

    // Draw the speaker name
    let speaker = character_store
        .0
        .get(&dialogue.character_id.clone().unwrap_or(String::from("")))
        .expect("Character not found!");

    let speaker_name_text = commands
        .spawn((
            Text::new("- ".to_owned() + &speaker.display_name.clone()),
            Name::new("Dialogue Speaker Text"),
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
