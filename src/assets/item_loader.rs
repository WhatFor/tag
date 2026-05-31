use std::collections::HashMap;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::item_asset_loader::{ItemAssetLoader, ItemData};
use crate::sets::PlayingSet;
use crate::state::GameState;

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(
        paths("items/iron_sword.item.ron", "items/potion.item.ron"),
        collection(typed)
    )]
    handles: Vec<Handle<ItemData>>,
}

#[derive(Debug)]
pub struct ItemDef {
    pub name: String,
    pub description: String,
}

#[derive(Resource, Default, Debug)]
pub struct ItemStore(pub HashMap<String, ItemDef>);

pub struct ItemLoaderPlugin;

///Loads item.ron assets.
impl Plugin for ItemLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemData>();
        app.init_asset_loader::<ItemAssetLoader>();
        app.init_resource::<ItemStore>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<ItemAssets>(),
        );

        app.add_systems(
            OnEnter(GameState::Playing),
            add_item_store.in_set(PlayingSet::SpawnCharacters),
        );
    }
}

fn add_item_store(
    items: Res<ItemAssets>,
    item_data: Res<Assets<ItemData>>,
    mut item_store: ResMut<ItemStore>,
) {
    for handle in &items.handles {
        let Some(data) = item_data.get(handle) else {
            continue;
        };

        info!("Parsed Item {}...", data.name);

        item_store.0.insert(
            data.name.clone(),
            ItemDef {
                name: data.name.clone(),
                description: data.description.clone(),
            },
        );
    }
}
