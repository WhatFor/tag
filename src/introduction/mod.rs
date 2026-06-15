use crate::prelude::*;
use bevy::prelude::*;

pub struct IntroductionPlugin;

impl Plugin for IntroductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Introduction), init);
    }
}

fn init(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            Name::new("Introduction Container"),
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
            DespawnOnExit(GameState::Introduction),
        ))
        .with_children(|p| {
            p.spawn((
                Name::from("Introduction Title"),
                Text::from("TAG"),
                fonts.narration_font.clone(),
                fonts.narration_color.clone(),
            ));

            p.spawn((
                Name::from("Introduction Text Container"),
                Node {
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.),
                    padding: UiRect::axes(Val::Percent(5.), Val::Percent(0.)),
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(Text::from(
                    "Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 ",
                ));
                p.spawn(Text::from(
                    "Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 ",
                ));
                p.spawn(Text::from(
                    "Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 ",
                ));
            });

            p.spawn(button("Continue")).observe(
                |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::Playing);
                },
            );
        });
}
