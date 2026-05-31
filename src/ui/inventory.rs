use bevy::prelude::*;

use crate::{global::PausableSystems, state::GameState};

#[derive(Component)]
struct InventoryButton;

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), button_init);

        app.add_systems(
            Update,
            button_interaction
                .run_if(in_state(GameState::Playing))
                .in_set(PausableSystems),
        );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_init(mut commands: Commands) {
    commands.spawn((
        Button,
        InventoryButton,
        DespawnOnExit(GameState::Playing),
        Name::new("Inventory Button"),
        Node {
            // Position
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(50.0),
            // Size
            width: px(200),
            height: px(65),
            // Centre text
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new("Inventory"),
            Name::new("Inventory Button Text"),
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
    ));
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // TODO: Open inventory
            }
        }
    }
}
