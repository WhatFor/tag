use bevy::log::*;
use bevy::prelude::*;

use crate::ui::area_ui::AreaUIPlugin;
use crate::{global::PendingAssets, state::GameState};

mod area_ui;

#[derive(Resource)]
pub struct FontAssets {
    pub title_font: TextFont,
    pub title_color: TextColor,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AreaUIPlugin);

        app.add_systems(OnEnter(GameState::Initialising), setup_fonts);
    }
}

fn setup_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_tracker: ResMut<PendingAssets>,
) {
    info!("Beginning to load Font assets...");

    let font = asset_server.load("fonts/iglesia.otf");
    asset_tracker.0.push(font.clone().untyped());

    let title_font = TextFont {
        font: font.clone(),
        font_size: 90.0,
        ..default()
    };

    let title_color = TextColor(Color::srgb(0.8, 0.9, 1.0));

    commands.insert_resource(FontAssets {
        title_font,
        title_color,
    });
}
