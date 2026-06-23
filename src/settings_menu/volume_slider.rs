use bevy::prelude::*;

use bevy::ui_widgets::Slider;
use bevy::ui_widgets::SliderStep;
use bevy::ui_widgets::SliderThumb;
use bevy::ui_widgets::SliderValue;

pub const TRACK_WIDTH: f32 = 200.0;
pub const TRACK_HEIGHT: f32 = 25.0;
pub const THUMB_WIDTH: f32 = 10.0;

#[derive(Component, Clone, Copy)]
pub enum VolumeControl {
    Master,
    Music,
    Ambience,
    Sfx,
}

pub fn volume_slider(control: VolumeControl, initial: f32) -> impl Bundle {
    (
        control,
        Slider::default(),
        SliderValue(initial),
        SliderStep(0.05),
        BackgroundColor(Color::srgb(0., 0., 0.)),
        Node {
            width: Val::Px(TRACK_WIDTH),
            height: Val::Px(TRACK_HEIGHT),
            ..default()
        },
        children![(
            SliderThumb,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(THUMB_WIDTH),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgb(1., 1., 1.))
        )],
    )
}
