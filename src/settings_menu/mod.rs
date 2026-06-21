use crate::prelude::*;
use bevy::prelude::*;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SettingsMenu), init);
        // app.add_systems(OnEnter(GameState::GameOver), reset_to_menu);
    }
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            Node {
                // center button
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                ..default()
            },
            GlobalZIndex(LAYER_HUD),
            Name::new("Settings Menu Container"),
            DespawnOnExit(GameState::SettingsMenu),
        ))
        .with_children(|p| {
            p.spawn(button("Back")).observe(
                |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::MainMenu);
                },
            );
        });
}
