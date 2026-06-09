use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::item_asset_loader::{ItemAssetLoader, ItemData};
use std::collections::HashMap;

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(
        paths("items/iron_sword.item.ron", "items/potion.item.ron"),
        collection(typed)
    )]
    handles: Vec<Handle<ItemData>>,
}

#[derive(Debug, Clone)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stackable: bool,
}

#[derive(Resource, Default, Debug)]
pub struct ItemStore(pub HashMap<String, ItemDef>);

pub struct ItemLoaderPlugin;

impl Plugin for ItemLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemData>();
        app.init_asset_loader::<ItemAssetLoader>();
        app.init_resource::<ItemStore>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<ItemAssets>(),
        );

        app.add_systems(OnExit(GameState::Initialising), add_item_store);
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
            data.id.clone(),
            ItemDef {
                id: data.id.clone(),
                name: data.name.clone(),
                description: data.description.clone(),
                stackable: data.stackable,
            },
        );
    }
}
