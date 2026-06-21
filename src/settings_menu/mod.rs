use crate::prelude::*;
use bevy::audio::Volume;
use bevy::prelude::*;

use crate::settings_menu::volume_slider::VolumeControl;
use crate::settings_menu::volume_slider::volume_slider;
use bevy::ui_widgets::SliderRange;
use bevy::ui_widgets::SliderThumb;
use bevy::ui_widgets::SliderValue;
use bevy::ui_widgets::slider_self_update;

mod volume_slider;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SettingsMenu), init);

        app.add_observer(slider_self_update);
        app.add_systems(Update, slider_changed);
    }
}

fn init(mut commands: Commands, audio_settings: Res<AudioSettings>, font_store: Res<FontAssets>) {
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
            for (label, control, initial) in [
                (
                    "Master",
                    VolumeControl::Master,
                    audio_settings.master_volume,
                ),
                ("Music", VolumeControl::Music, audio_settings.music_volume),
                (
                    "Ambience",
                    VolumeControl::Ambience,
                    audio_settings.ambience_volume,
                ),
                ("SFX", VolumeControl::Sfx, audio_settings.sfx_volume),
            ] {
                p.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.),
                        ..default()
                    },
                    children![
                        (
                            Text::from(label),
                            font_store.ui_font.clone(),
                            font_store.ui_color
                        ),
                        volume_slider(control, initial)
                    ],
                ));
            }

            p.spawn(button("Back")).observe(
                |_: On<Pointer<Click>>,
                 mut commands: Commands,
                 mut next_state: ResMut<NextState<GameState>>| {
                    commands.trigger(SettingsUpdated);
                    next_state.set(GameState::MainMenu);
                },
            );
        });
}

fn slider_changed(
    sliders: Query<(&SliderValue, &SliderRange, &VolumeControl, &Children), Changed<SliderValue>>,
    mut thumbs: Query<&mut Node, With<SliderThumb>>,
    mut settings: ResMut<AudioSettings>,
    mut global: ResMut<GlobalVolume>,
) {
    for (value, range, control, children) in &sliders {
        match control {
            VolumeControl::Master => {
                global.volume = Volume::Linear(value.0);
                settings.master_volume = value.0;
            }
            VolumeControl::Music => settings.music_volume = value.0,
            VolumeControl::Ambience => settings.ambience_volume = value.0,
            VolumeControl::Sfx => settings.sfx_volume = value.0,
        }

        let frac = range.thumb_position(value.0);

        for &child in children {
            if let Ok(mut node) = thumbs.get_mut(child) {
                node.left = Val::Percent(frac * 100.0);
            }
        }
    }
}
