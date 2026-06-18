use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::ui::area::AreaUIPlugin;
use crate::ui::content::ContentUIPlugin;
use crate::ui::inventory::InventoryUIPlugin;
use crate::ui::layout::UILayoutPlugin;
use crate::ui::widgets::WidgetsPlugin;

pub mod events;
pub mod inventory;
pub mod layers;
pub mod layout;
pub mod widgets;

mod area;
mod content;

#[derive(AssetCollection, Resource)]
pub struct FontHandles {
    #[asset(path = "fonts/iglesia.otf")]
    pub title: Handle<Font>,
    #[asset(path = "fonts/gentium.ttf")]
    pub narration: Handle<Font>,
    #[asset(path = "fonts/gentium_italic.ttf")]
    pub dialogue: Handle<Font>,
    #[asset(path = "fonts/bebas.otf")]
    pub ui: Handle<Font>,
}

#[derive(Resource)]
pub struct FontAssets {
    pub title_font: TextFont,
    pub title_color: TextColor,

    pub narration_font: TextFont,
    pub narration_color: TextColor,

    pub dialogue_font: TextFont,
    pub dialogue_color: TextColor,

    pub ui_font: TextFont,
    pub ui_color: TextColor,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WidgetsPlugin);
        app.add_plugins(UILayoutPlugin);
        app.add_plugins(AreaUIPlugin);
        app.add_plugins(ContentUIPlugin);
        app.add_plugins(InventoryUIPlugin);

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising)
                .load_collection::<FontHandles>()
                .finally_init_resource::<FontAssets>(),
        );
    }
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let fonts = world.resource::<FontHandles>();

        Self {
            title_font: TextFont {
                font: fonts.title.clone(),
                font_size: 90.0,
                ..default()
            },
            title_color: TextColor(Color::srgb(0.8, 0.9, 1.0)),

            narration_font: TextFont {
                font: fonts.narration.clone(),
                font_size: 45.0,
                ..default()
            },
            narration_color: TextColor(Color::srgb(1.0, 0.95, 1.0)),

            dialogue_font: TextFont {
                font: fonts.dialogue.clone(),
                font_size: 45.0,
                ..default()
            },
            dialogue_color: TextColor(Color::srgb(0.9, 0.475, 0.425)),

            ui_font: TextFont {
                font: fonts.ui.clone(),
                font_size: 30.0,
                ..default()
            },
            ui_color: TextColor(Color::srgb(0.6, 0.65, 0.6)),
        }
    }
}
