use crate::prelude::*;
use bevy::prelude::*;

use bevy::audio::PlaybackMode;
use bevy::audio::Volume;

#[derive(Event)]
pub struct PlaySoundtrack;

#[derive(Event)]
pub struct StopSoundtrack;

#[derive(Component)]
struct Soundtrack;

#[derive(Component)]
struct FadeIn(Timer);

#[derive(Component)]
struct FadeOut(Timer);

const FADE_TIME: f32 = 2.;

pub struct SoundtrackPlugin;

impl Plugin for SoundtrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(play);
        app.add_observer(stop);
        app.add_systems(Update, (fade_in, fade_out));
    }
}

fn play(
    _: On<PlaySoundtrack>,
    mut commands: Commands,
    audio_store: Res<AudioAssets>,
    soundtrack: Query<Entity, With<Soundtrack>>,
) {
    let Some(placeholder) = audio_store.background.get("placeholder") else {
        warn!("Unable to find placeholder track!");
        return;
    };

    for track in soundtrack.iter() {
        commands
            .entity(track)
            .insert(FadeOut(Timer::from_seconds(FADE_TIME, TimerMode::Once)));
    }

    info!("Starting soundtrack...");

    commands.spawn((
        Name::from("Soundtrack"),
        Soundtrack,
        AudioPlayer(placeholder.clone()),
        FadeIn(Timer::from_seconds(FADE_TIME, TimerMode::Once)),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(1.),
            ..default()
        },
    ));
}

fn stop(
    _: On<StopSoundtrack>,
    mut commands: Commands,
    soundtrack: Query<Entity, With<Soundtrack>>,
) {
    info!("Stopping soundtrack...");

    for track in soundtrack.iter() {
        commands
            .entity(track)
            .insert(FadeOut(Timer::from_seconds(FADE_TIME, TimerMode::Once)));
    }
}

fn fade_in(
    mut commands: Commands,
    mut tracks: Query<(Entity, &mut AudioSink, &mut FadeIn)>,
    time: Res<Time>,
) {
    for (entity, mut audio, mut fade) in &mut tracks {
        fade.0.tick(time.delta());

        audio.set_volume(Volume::SILENT.fade_towards(Volume::Linear(1.), fade.0.fraction()));

        if fade.0.is_finished() {
            audio.set_volume(Volume::Linear(1.));
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn fade_out(
    mut commands: Commands,
    mut tracks: Query<(Entity, &mut AudioSink, &mut FadeOut)>,
    time: Res<Time>,
) {
    for (entity, mut audio, mut fade) in &mut tracks {
        fade.0.tick(time.delta());

        audio.set_volume(Volume::Linear(1.).fade_towards(Volume::SILENT, fade.0.fraction()));

        if fade.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
