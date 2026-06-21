use bevy::prelude::*;

use bevy::ui_widgets::Slider;
use bevy::ui_widgets::SliderStep;
use bevy::ui_widgets::SliderThumb;
use bevy::ui_widgets::SliderValue;

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
            width: Val::Percent(100.),
            height: Val::Px(20.),
            ..default()
        },
        children![(
            SliderThumb,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(8.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgb(1., 1., 1.))
        )],
    )
}
