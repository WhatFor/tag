use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenRoot;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GameArea;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HudAreaTop;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HudAreaBottom;

pub struct UILayoutPlugin;

impl Plugin for UILayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initialising), spawn_layout);
    }
}

fn spawn_layout(mut commands: Commands) {
    commands.spawn((
        ScreenRoot,
        Name::new("Screen Root"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                HudAreaTop,
                Name::new("HUD Area Top"),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(120.),
                    flex_shrink: 0.,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(30.), Val::Px(0.)),
                    ..default()
                }
            ),
            (
                GameArea,
                Name::new("Main Game Area"),
                Node {
                    padding: UiRect::axes(Val::Percent(5.), Val::Percent(0.)),
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    ..default()
                }
            ),
            (
                HudAreaBottom,
                Name::new("HUD Area Bottom"),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(120.),
                    flex_shrink: 0.,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(30.), Val::Px(0.)),
                    ..default()
                }
            )
        ],
    ));
}
