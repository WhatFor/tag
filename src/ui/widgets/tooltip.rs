use crate::prelude::*;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;

const FONT_SIZE: f32 = 20.0;
const FADE_SECONDS: f32 = 0.15 * GLOBAL_ANIMATION_SPEED;

const BG_COLOUR: Color = Color::srgb(0.15, 0.15, 0.15);
const BORDER_COLOUR: Color = Color::srgb(0.25, 0.25, 0.25);

const PADDING: f32 = 10.0;
const MAX_WIDTH: f32 = 280.0;
const OFFSET: f32 = 20.0;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Tooltip {
    // todo: make more flexible
    pub text: String,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Component)]
struct TooltipElement;

// TODO: Break out into a generic 'fade in' animation
#[derive(Component)]
struct TooltipFadeIn(Timer);

pub struct TooltipWidgetPlugin;

impl Plugin for TooltipWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (follow, clamp, fade_in).chain());
        app.add_observer(on_mouse_over);
        app.add_observer(on_mouse_out);
    }
}

fn follow(
    window: Single<&Window, With<PrimaryWindow>>,
    tooltip: Single<(&ComputedNode, &mut Node), With<TooltipElement>>,
) {
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let (computed, mut node) = tooltip.into_inner();
    let size = computed.size();

    if size.x <= 0.0 || size.y <= 0.0 {
        return;
    }

    node.left = Val::Px(cursor.x + OFFSET);
    node.top = Val::Px(cursor.y + OFFSET - size.y);
}

fn clamp(
    window: Single<&Window, With<PrimaryWindow>>,
    tooltip: Single<(&ComputedNode, &mut Node), With<TooltipElement>>,
) {
    let (computed, mut node) = tooltip.into_inner();
    let size = computed.size();

    if size.x <= 0.0 || size.y <= 0.0 {
        return;
    }

    let margin = 8.0;
    let max_left = (window.width() - size.x - margin).max(margin);
    let max_top = (window.height() - size.y - margin).max(margin);

    if let Val::Px(left) = node.left {
        if left > max_left {
            node.left = Val::Px(max_left);
        }
    }

    if let Val::Px(top) = node.top {
        if top > max_top {
            node.top = Val::Px(max_top);
        }
    }
}

fn fade_in(
    time: Res<Time>,
    mut commands: Commands,
    mut texts: Query<&mut TextColor>,
    mut roots: Query<(
        Entity,
        &ComputedNode,
        &mut TooltipFadeIn,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
) {
    for (entity, computed, mut fade, mut bg, mut border, children) in &mut roots {
        if computed.size().y <= 0.0 {
            // Don't start fading until drawn
            continue;
        }

        fade.0.tick(time.delta());

        let alpha = fade.0.fraction().min(1.0);

        bg.0.set_alpha(alpha);
        border.set_all(BORDER_COLOUR.with_alpha(alpha));

        for &child in children {
            if let Ok(mut text) = texts.get_mut(child) {
                text.0.set_alpha(alpha);
            }
        }

        if fade.0.just_finished() {
            commands.entity(entity).remove::<TooltipFadeIn>();
        }
    }
}

fn on_mouse_over(
    trigger: On<Pointer<Over>>,
    tooltips: Query<&Tooltip>,
    existing: Query<Entity, With<TooltipElement>>,
    mut commands: Commands,
) {
    let entity = trigger.event_target();

    let Ok(tooltip) = tooltips.get(entity) else {
        return;
    };

    // Clear any existing, just to be sure
    for e in &existing {
        commands.entity(e).try_despawn();
    }

    let pos = trigger.pointer_location.position;

    commands.spawn((
        TooltipElement,
        TooltipFadeIn(Timer::from_seconds(FADE_SECONDS, TimerMode::Once)),
        Name::new("Tooltip"),
        Pickable::IGNORE,
        GlobalZIndex(LAYER_TOOLTIP),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            max_width: Val::Px(MAX_WIDTH),
            padding: UiRect::all(Val::Px(PADDING)),
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        BackgroundColor(BG_COLOUR.with_alpha(0.0)),
        BorderColor::all(BORDER_COLOUR.with_alpha(0.0)),
        children![(
            Text::new(tooltip.text.clone()),
            Pickable::IGNORE,
            TextFont {
                font_size: FONT_SIZE,
                ..default()
            },
            TextColor(Color::srgba(0.9, 0.9, 0.9, 0.0)),
        )],
    ));
}

fn on_mouse_out(
    trigger: On<Pointer<Out>>,
    tooltips: Query<&Tooltip>,
    existing: Query<Entity, With<TooltipElement>>,
    mut commands: Commands,
) {
    let Ok(_) = tooltips.get(trigger.event_target()) else {
        return;
    };

    for e in &existing {
        commands.entity(e).try_despawn();
    }
}
