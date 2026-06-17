use bevy::prelude::*;

use crate::ui::FontAssets;

const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

const PADDING_X: f32 = 20.;
const PADDING_Y: f32 = 10.;

#[derive(Component)]
pub struct StyledButton;

#[derive(Component)]
pub struct StyledButtonText;

pub struct ButtonWidgetPlugin;

impl Plugin for ButtonWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_create);

        app.add_observer(on_mouse_hover);
        app.add_observer(on_mouse_unhover);

        app.add_observer(on_mouse_down);
        app.add_observer(on_mouse_up);
    }
}

pub fn button(label: impl Into<String>) -> impl Bundle {
    let label = label.into();
    (
        Button,
        StyledButton,
        Name::new(format!("{} Button", label.clone())),
        BackgroundColor(NORMAL),
        Node {
            padding: UiRect {
                left: Val::Px(PADDING_X),
                right: Val::Px(PADDING_X),
                top: Val::Px(PADDING_Y),
                bottom: Val::Px(PADDING_Y),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Name::new(format!("{} Button Text", label.clone())),
            Text::new(label),
            StyledButtonText,
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

fn on_create(
    add: On<Add, StyledButtonText>,
    fonts: Res<FontAssets>,
    mut query: Query<(&mut TextFont, &mut TextColor), With<StyledButtonText>>,
) {
    if let Ok((mut button_font, mut button_color)) = query.get_mut(add.entity) {
        button_font.font = fonts.ui_font.font.clone();
        button_color.0 = fonts.ui_color.0.clone();
    }
}

fn on_mouse_hover(
    trigger: On<Pointer<Over>>,
    mut query: Query<&mut BackgroundColor, With<StyledButton>>,
) {
    if let Ok(mut background_colour) = query.get_mut(trigger.event_target()) {
        *background_colour = BackgroundColor(HOVERED);
    }
}

fn on_mouse_unhover(
    trigger: On<Pointer<Out>>,
    mut query: Query<&mut BackgroundColor, With<StyledButton>>,
) {
    if let Ok(mut background_colour) = query.get_mut(trigger.event_target()) {
        *background_colour = BackgroundColor(NORMAL);
    }
}

fn on_mouse_down(
    trigger: On<Pointer<Press>>,
    mut query: Query<&mut BackgroundColor, With<StyledButton>>,
) {
    if let Ok(mut background_colour) = query.get_mut(trigger.event_target()) {
        *background_colour = BackgroundColor(PRESSED);
    }
}

fn on_mouse_up(
    trigger: On<Pointer<Release>>,
    mut query: Query<&mut BackgroundColor, With<StyledButton>>,
) {
    if let Ok(mut background_colour) = query.get_mut(trigger.event_target()) {
        *background_colour = BackgroundColor(HOVERED);
    }
}
