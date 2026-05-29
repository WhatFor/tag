use bevy::log::*;
use bevy::{asset::LoadedFolder, prelude::*};

use crate::global::PendingAssets;
use crate::sets::PlayingSet;
use crate::{
    assets::loader::{AreaAssetLoader, AreaData},
    state::GameState,
    world::components::Area,
};

pub mod loader;

#[derive(Resource)]
pub struct AreaFolder(pub Handle<LoadedFolder>);

pub struct AreaLoaderPlugin;

///Loads Area .ron assets.
///
/// When the game launches, its's first in GameState::Initialising.
/// This triggers the 'load_areas' system.
/// On each Update, we run 'check_progress' to wait until all assets are loaded.
/// When they are loaded, we transition to the GameState::MainMenu.
/// Eventually, when we enter GameState::Playing, we spawn the Area assets.
impl Plugin for AreaLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AreaData>();
        app.init_asset_loader::<AreaAssetLoader>();

        app.add_systems(OnEnter(GameState::Initialising), load_areas);

        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_areas.in_set(PlayingSet::SpawnWorld),
        );
    }
}

const AREAS_ASSET_PATH: &str = "areas";

fn load_areas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_tracker: ResMut<PendingAssets>,
) {
    info!("Beginning to load Area assets from folder...");
    let areas_handle = asset_server.load_folder(AREAS_ASSET_PATH);
    asset_tracker.0.push(areas_handle.clone().untyped());
    commands.insert_resource(AreaFolder(areas_handle));
}

fn spawn_areas(mut commands: Commands, areas: Res<Assets<AreaData>>) {
    for (_, area_data) in areas.iter() {
        info!("Spawning {} ({})...", area_data.name, area_data.id);
        commands.spawn((
            Area,
            crate::components::Name(area_data.name.clone()),
            crate::world::components::AreaId(area_data.id.clone()),
        ));
    }

    commands.remove_resource::<AreaFolder>();
}
