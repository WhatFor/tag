use crate::prelude::*;
use bevy::prelude::*;

use crate::game::resources::HardcoreMode;
use crate::ui::layout::GameArea;
use bevy::ui::Checked;
use bevy::ui_widgets::checkbox_self_update;

pub struct IntroductionPlugin;

impl Plugin for IntroductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Introduction), init);
    }
}

const INTRODUCTION_LINES: &[&str] = &[
    "You remember the sensation of grass under your bare feet and the warmth of the evening sun.",
    "The familiar sounds, bringing calm, to the place you called home.",
    "But that was before.",
    "Now, your senses fail you. You see nothing, and your limbs fail to find solid ground.",
    "There is no up, no down. No time, nor sound.",
    "Until a distant, deep and mountainous voice echoes through the void.",
    "You try to call out to the stranger, sensing a way out...",
    "You wake.",
];

fn init(mut commands: Commands, game_area: Single<Entity, With<GameArea>>, fonts: Res<FontAssets>) {
    commands
        .spawn((
            Name::new("Introduction Container"),
            ChildOf(game_area.entity()),
            Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(30.),
                ..default()
            },
            DespawnOnExit(GameState::Introduction),
        ))
        .with_children(|p| {
            p.spawn((
                Name::from("Introduction Title"),
                Text::from("Welcome"),
                fonts.narration_font.clone(),
                fonts.narration_color.clone(),
            ));

            p.spawn((
                Name::from("Introduction Text Container"),
                Node {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.),
                    padding: UiRect::axes(Val::Percent(5.), Val::Percent(0.)),
                    ..default()
                },
            ))
            .with_children(|p| {
                for line in INTRODUCTION_LINES {
                    p.spawn((
                        Text::from(*line),
                        fonts.narration_font.clone().with_font_size(24.),
                        fonts.narration_color.clone(),
                    ));
                }
            });

            let mut hardcore_box = Entity::PLACEHOLDER;

            p.spawn((
                Name::from("Hardcore Row"),
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.),
                    ..default()
                },
            ))
            .with_children(|row| {
                hardcore_box = row.spawn(checkbox()).observe(checkbox_self_update).id();
                row.spawn((
                    Text::from("Hardcore mode"),
                    Tooltip::basic(
                        "If enabled, death is permanent and you cannot load checkpoints.",
                    ),
                    fonts.ui_font.clone().with_font_size(24.),
                    fonts.ui_color.clone(),
                ));
            });

            p.spawn(button("Continue")).observe(
                move |_: On<Pointer<Click>>,
                      mut commands: Commands,
                      mut next_state: ResMut<NextState<GameState>>,
                      checked: Query<(), With<Checked>>| {
                    let hardcore = checked.get(hardcore_box).is_ok();
                    commands.insert_resource(HardcoreMode(hardcore));

                    next_state.set(GameState::Playing);
                },
            );
        });
}
