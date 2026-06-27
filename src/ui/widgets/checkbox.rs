use bevy::prelude::*;

use bevy::ui::Checked;
use bevy::ui_widgets::Checkbox;

const BOX_SIZE: f32 = 26.;
const BORDER: Color = Color::srgb(0.6, 0.6, 0.6);
const EMPTY: Color = Color::NONE;
const FILL: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct CheckboxWidgetPlugin;

impl Plugin for CheckboxWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::ui_widgets::CheckboxPlugin);
        app.add_observer(on_checked_added);
        app.add_observer(on_checked_removed);
    }
}

pub fn checkbox() -> impl Bundle {
    (
        Checkbox,
        Name::new("Checkbox"),
        Node {
            width: Val::Px(BOX_SIZE),
            height: Val::Px(BOX_SIZE),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor::all(BORDER),
        BackgroundColor(EMPTY),
    )
}

fn on_checked_added(add: On<Add, Checked>, mut q: Query<&mut BackgroundColor, With<Checkbox>>) {
    if let Ok(mut bg) = q.get_mut(add.entity) {
        *bg = BackgroundColor(FILL);
    }
}

fn on_checked_removed(
    rem: On<Remove, Checked>,
    mut q: Query<&mut BackgroundColor, With<Checkbox>>,
) {
    if let Ok(mut bg) = q.get_mut(rem.entity) {
        *bg = BackgroundColor(EMPTY);
    }
}
