use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::enemy_asset_loader::EnemyAssetLoader;
use crate::assets::enemy_asset_loader::EnemyData;

use std::collections::HashMap;

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(paths("enemies/small_rat.enem.ron"), collection(typed))]
    handles: Vec<Handle<EnemyData>>,
}

#[derive(Debug)]
pub struct EnemyDef {
    pub id: String,
    pub display_name: String,
    pub max_health: i32,
    pub stats: Stats,
    pub moves: Vec<EnemyMove>,
    pub loot: Vec<EnemyLoot>,
    pub gold: u32,
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct EnemyStore(pub HashMap<String, EnemyDef>);

pub struct EnemyLoaderPlugin;

impl Plugin for EnemyLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<EnemyData>();
        app.init_asset_loader::<EnemyAssetLoader>();
        app.init_resource::<EnemyStore>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<EnemyAssets>(),
        );

        app.add_systems(OnExit(GameState::Initialising), add_resource);
    }
}

fn add_resource(
    enemies: Res<EnemyAssets>,
    enemy_data: Res<Assets<EnemyData>>,
    mut enemy_store: ResMut<EnemyStore>,
) {
    for handle in &enemies.handles {
        let Some(data) = enemy_data.get(handle) else {
            continue;
        };

        info!("Parsed Enemy {}...", data.id);

        enemy_store.insert(
            data.id.clone(),
            EnemyDef {
                id: data.id.clone(),
                display_name: data.display_name.clone(),
                max_health: data.max_health,
                stats: data.stats,
                moves: data.moves.clone(),
                loot: data.loot.clone(),
                gold: data.gold,
            },
        );
    }
}
