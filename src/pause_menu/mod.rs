use bevy::prelude::*;

use crate::{global::Pause, pause_menu::resources::MenuData, state::GameState};

mod resources;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Pause(true)), spawn_pause_menu);
        app.add_systems(Update, pause_menu.run_if(in_state(Pause(true))));
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn spawn_pause_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn((
            GlobalZIndex(2),
            DespawnOnExit(Pause(true)),
            Node {
                // center button
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![(
                Button,
                Node {
                    width: px(200),
                    height: px(65),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Main Menu"),
                    TextLayout {
                        linebreak: LineBreak::NoWrap,
                        ..default()
                    },
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            )],
        ))
        .id();

    commands.insert_resource(MenuData { button_entity });
}

fn pause_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_state: ResMut<NextState<Pause>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                next_pause_state.set(Pause(false));
                next_game_state.set(GameState::MainMenu);
            }
        }
    }
}
