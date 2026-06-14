use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::area_asset_loader::{AreaAssetLoader, AreaData};

#[derive(AssetCollection, Resource)]
pub struct AreaAssets {
    #[asset(
        paths(
            "areas/01.area.ron",
            "areas/02.area.ron",
            "areas/03.area.ron",
            "areas/04.area.ron"
        ),
        collection(typed)
    )]
    handles: Vec<Handle<AreaData>>,
}

pub struct AreaLoaderPlugin;

impl Plugin for AreaLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AreaData>();
        app.init_asset_loader::<AreaAssetLoader>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<AreaAssets>(),
        );

        app.add_systems(
            OnExit(GameState::Initialising),
            spawn_areas.in_set(PlayingSet::SpawnAreas),
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
            Name::new(format!("Area {}", data.id)),
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
