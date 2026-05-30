use bevy::prelude::*;

use crate::state::GameState;
use crate::ui::FontAssets;
use crate::world::components::{Area, AreaNarration};
use crate::world::events::PlayerEnteredArea;

#[derive(Component)]
struct DialogueNode;

#[derive(Component)]
struct NarrationReveal {
    pub next_index: usize,
    pub timer: Timer,
}

#[derive(Component)]
struct NextChar(pub Option<Entity>);

#[derive(Component)]
struct AnimatingOpacity(f32);

#[derive(Component)]
struct AnimationComplete;

impl Default for NarrationReveal {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        timer.pause();

        Self {
            next_index: 0,
            timer,
        }
    }
}

pub struct DialogueUIPlugin;

impl Plugin for DialogueUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init);
        //app.add_systems(Update, reveal_lines.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, on_char_done.run_if(in_state(GameState::Playing)));
        app.add_observer(on_player_enter_area);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        DialogueNode,
        NarrationReveal::default(),
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
    mut dialogue: Query<(Entity, &mut NarrationReveal), With<DialogueNode>>,
    fonts: Res<FontAssets>,
) {
    let Ok(narration) = all_area_narration.get(event.0) else {
        return;
    };

    let Ok((dialogue_node, mut reveal_info)) = dialogue.single_mut() else {
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
                    TextColor(fonts.narration_color.with_alpha(0.0)),
                ))
                .id()
        })
        .collect();

    commands.entity(dialogue_node).replace_children(&lines);

    reveal_info.next_index = 0;
    reveal_info.timer.reset();
    reveal_info.timer.unpause();
}

// When the AnimationComplete is finished on a char, kick off the next one.
fn on_char_done(mut commands: Commands, query: Query<&NextChar, Added<AnimationComplete>>) {
    for next in &query {
        if let Some(next_entity) = next.0 {
            commands.entity(next_entity).insert(AnimatingOpacity(0.0));
        }
    }
}

/*fn reveal_lines(
    time: Res<Time>,
    mut dialogue: Query<(Entity, &mut NarrationReveal), With<DialogueNode>>,
    children_query: Query<&Children>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let Ok((dialogue_node, mut reveal_info)) = dialogue.single_mut() else {
        return;
    };

    if !reveal_info.timer.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(children) = children_query.get(dialogue_node) else {
        return;
    };

    if reveal_info.next_index >= children.len() {
        return;
    }

    if let Ok(mut visibility) = visibility_query.get_mut(children[reveal_info.next_index]) {
        *visibility = Visibility::Visible;
    }

    reveal_info.next_index += 1;
}*/
