use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::sets::PlayingSet;
use crate::world::components::{AreaDialogue, AreaExits, AreaNarration};
use crate::{
    assets::area_asset_loader::{AreaAssetLoader, AreaData},
    state::GameState,
    world::components::Area,
};

#[derive(AssetCollection, Resource)]
pub struct AreaAssets {
    #[asset(paths("areas/01.area.ron", "areas/02.area.ron"), collection(typed))]
    handles: Vec<Handle<AreaData>>,
}

pub struct AreaLoaderPlugin;

///Loads Area .ron assets.
///
/// When the game launches, its's first in GameState::Initialising.
/// This triggers the 'load_areas' system.
/// On each Update, we run 'check_progress' to wait until all assets are loaded.
/// Eventually, when we enter GameState::Playing, we spawn the Area assets.
impl Plugin for AreaLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AreaData>();
        app.init_asset_loader::<AreaAssetLoader>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<AreaAssets>(),
        );

        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_areas.in_set(PlayingSet::SpawnWorld),
        );
    }
}

fn spawn_areas(mut commands: Commands, areas: Res<AreaAssets>, area_data: Res<Assets<AreaData>>) {
    for handle in &areas.handles {
        let Some(data) = area_data.get(handle) else {
            continue;
        };

        info!("Spawning Area {}...", data.id);

        commands.spawn((
            Area,
            AreaExits(data.exits.clone()),
            AreaNarration {
                lines: data.narration.lines.clone(),
            },
            AreaDialogue {
                character_id: data.dialogue.character_id.clone(),
                lines: data.dialogue.lines.clone(),
            },
            crate::components::DisplayName(data.name.clone()),
            crate::world::components::AreaId(data.id.clone()),
        ));
    }
}
