use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use bevy::platform::collections::HashMap;
use bevy_asset_loader::mapped::AssetFileStem;

use crate::assets::item_asset_loader::ItemAssetLoader;
use crate::assets::item_asset_loader::ItemData;

#[derive(AssetCollection, Resource)]
pub struct ItemIconAssets {
    #[asset(
        paths(
            "icons/_placeholder.png",
            "icons/bronze_sword.png",
            "icons/iron_sword.png",
            "icons/potion.png"
        ),
        collection(typed, mapped)
    )]
    icons: HashMap<AssetFileStem, Handle<Image>>,
}

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(
        paths(
            "items/bronze_sword.item.ron",
            "items/iron_sword.item.ron",
            "items/potion.item.ron"
        ),
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
    pub slot: Option<EquipmentSlot>,
    pub icon: Handle<Image>,
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct ItemStore(pub HashMap<String, ItemDef>);

pub struct ItemLoaderPlugin;

impl Plugin for ItemLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemData>();
        app.init_asset_loader::<ItemAssetLoader>();
        app.init_resource::<ItemStore>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising)
                .load_collection::<ItemAssets>()
                .load_collection::<ItemIconAssets>(),
        );

        app.add_systems(OnExit(GameState::Initialising), add_item_store);
    }
}

fn add_item_store(
    items: Res<ItemAssets>,
    item_data: Res<Assets<ItemData>>,
    mut item_store: ResMut<ItemStore>,
    icon_store: Res<ItemIconAssets>,
) {
    info!(
        "IconAssets keys: {:?}",
        icon_store.icons.keys().collect::<Vec<_>>()
    );

    let placeholder_icon = icon_store.icons.get("_placeholder").cloned();

    if placeholder_icon.is_none() {
        warn!("icons/_placeholder.png missing!");
    }

    for handle in &items.handles {
        let Some(data) = item_data.get(handle) else {
            continue;
        };

        info!("Parsed Item {}...", data.name);

        let icon_image = icon_store
            .icons
            .get(data.id.as_str())
            .cloned()
            .or_else(|| placeholder_icon.clone())
            .unwrap_or_default();

        item_store.insert(
            data.id.clone(),
            ItemDef {
                id: data.id.clone(),
                name: data.name.clone(),
                description: data.description.clone(),
                stackable: data.stackable,
                slot: data.slot,
                icon: icon_image,
            },
        );
    }
}
