use crate::prelude::*;
use crate::ui::layout::HudAreaTop;
use bevy::prelude::*;

use crate::ui::NarrationContainerNode;
use crate::ui::layout::GameArea;
use crate::ui::widgets::animation::text_fall::AnimateTextFall;
use crate::ui::widgets::animation::text_fall::AnimateTextFallComplete;

#[derive(Component)]
struct DialogueWrapperNode;

pub struct DialogueUIPlugin;

impl Plugin for DialogueUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_enter_area);
    }
}

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_dialogue: Query<&AreaDialogue, With<Area>>,
    character_store: Res<CharacterStore>,
    fonts: Res<FontAssets>,
    game_area: Single<Entity, With<GameArea>>,
    hud_area_top: Single<Entity, With<HudAreaTop>>,
) {
    // If we have no dialogue for the Area, skip.
    let Ok(dialogue) = all_area_dialogue.get(event.0) else {
        return;
    };

    if dialogue.lines.is_empty() {
        return;
    }

    // Draw the speaker name
    let speaker = character_store
        .0
        .get(&dialogue.character_id.clone().unwrap_or(String::from("")))
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
            DialogueWrapperNode,
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
}
