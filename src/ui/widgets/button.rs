use bevy::prelude::*;

const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct ButtonWidgetPlugin;

impl Plugin for ButtonWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, styled_button_interaction);
    }
}

pub fn button(label: impl Into<String>) -> impl Bundle {
    let label = label.into();
    (
        Button,
        StyledButton,
        Name::new(label.clone()),
        BackgroundColor(NORMAL),
        Node {
            // Size
            width: px(200),
            height: px(65),
            // Centre text
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(label),
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
    )
}

fn styled_button_interaction(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StyledButton>)>,
) {
    for (interaction, mut bg) in &mut q {
        *bg = BackgroundColor(match *interaction {
            Interaction::None => NORMAL,
            Interaction::Hovered => HOVERED,
            Interaction::Pressed => PRESSED,
        });
    }
}

#[derive(Component)]
pub struct StyledButton;
