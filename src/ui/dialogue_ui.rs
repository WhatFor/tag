use bevy::prelude::*;

use crate::state::GameState;
use crate::ui::FontAssets;
use crate::world::components::{Area, AreaNarration};
use crate::world::events::PlayerEnteredArea;

#[derive(Component)]
struct DialogueNode;

pub struct DialogueUIPlugin;

impl Plugin for DialogueUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init);
        app.add_observer(on_player_enter_area);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        DialogueNode,
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

fn on_player_enter_area(
    event: On<PlayerEnteredArea>,
    mut commands: Commands,
    all_area_narration: Query<&AreaNarration, With<Area>>,
    dialogue_node: Single<Entity, With<DialogueNode>>,
    fonts: Res<FontAssets>,
) {
    let Ok(narration) = all_area_narration.get(event.0) else {
        return;
    };

    let lines: Vec<Entity> = narration
        .lines
        .iter()
        .map(|l| {
            commands
                .spawn((
                    Text::new(l.clone()),
                    fonts.narration_font.clone(),
                    fonts.narration_color,
                ))
                .id()
        })
        .collect();

    commands.entity(*dialogue_node).replace_children(&lines);
}
