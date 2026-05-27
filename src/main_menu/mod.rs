use bevy::prelude::*;

use crate::state::GameState;
use resources::MenuData;

mod resources;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu);
        app.add_systems(Update, menu.run_if(in_state(GameState::MainMenu)));
        app.add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn((
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
                    width: px(150),
                    height: px(65),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Play"),
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

fn menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Playing);
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
    commands.remove_resource::<MenuData>();
}
