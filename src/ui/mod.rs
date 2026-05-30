use bevy::log::*;
use bevy::prelude::*;

use crate::ui::area_ui::AreaUIPlugin;
use crate::ui::continue_prompt_ui::ContinuePromptUIPlugin;
use crate::ui::dialogue_ui::DialogueUIPlugin;
use crate::{global::PendingAssets, state::GameState};

pub mod events;

mod area_ui;
mod continue_prompt_ui;
mod dialogue_ui;

#[derive(Resource)]
pub struct FontAssets {
    pub title_font: TextFont,
    pub title_color: TextColor,

    pub narration_font: TextFont,
    pub narration_color: TextColor,

    pub ui_font: TextFont,
    pub ui_color: TextColor,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AreaUIPlugin);
        app.add_plugins(DialogueUIPlugin);
        app.add_plugins(ContinuePromptUIPlugin);

        app.add_systems(OnEnter(GameState::Initialising), setup_fonts);
    }
}

fn setup_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_tracker: ResMut<PendingAssets>,
) {
    info!("Beginning to load Font assets...");

    // Load Title Font
    let title_font = asset_server.load("fonts/iglesia.otf");
    asset_tracker.0.push(title_font.clone().untyped());

    let title_font = TextFont {
        font: title_font.clone(),
        font_size: 90.0,
        ..default()
    };

    let title_color = TextColor(Color::srgb(0.8, 0.9, 1.0));

    // Load Narration Font
    let narration_font = asset_server.load("fonts/gentium.ttf");
    asset_tracker.0.push(narration_font.clone().untyped());

    let narration_font = TextFont {
        font: narration_font.clone(),
        font_size: 45.0,
        ..default()
    };

    let narration_color = TextColor(Color::srgb(1.0, 0.95, 1.0));

    // Load UI Font
    let ui_font = asset_server.load("fonts/bebas.otf");
    asset_tracker.0.push(ui_font.clone().untyped());

    let ui_font = TextFont {
        font: ui_font.clone(),
        font_size: 30.0,
        ..default()
    };

    let ui_color = TextColor(Color::srgb(0.6, 0.65, 0.6));

    commands.insert_resource(FontAssets {
        title_font,
        title_color,
        narration_font,
        narration_color,
        ui_font,
        ui_color,
    });
}
