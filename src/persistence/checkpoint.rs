use crate::persistence::CHECKPOINT_KEY_PREFIX;
use crate::prelude::*;
use bevy::prelude::*;

use anyhow::Context;

use crate::persistence::data::CheckpointData;
use crate::persistence::events::{LoadCheckpoint, SaveRequested};
use crate::persistence::resources::SaveBackend;

pub struct PersistenceCheckpointPlugin;

impl Plugin for PersistenceCheckpointPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_checkpoint_load);
    }
}

fn on_checkpoint_load(
    trigger: On<LoadCheckpoint>,
    mut commands: Commands,
    store: Res<SaveBackend>,
    areas: Query<(Entity, &AreaId), With<Area>>,
    mut player: Single<
        (
            Entity,
            &mut Health,
            &mut Inventory,
            &mut CurrentArea,
            &mut FullPathTaken,
        ),
        With<Player>,
    >,
) -> Result {
    let key = format!("{}{}", CHECKPOINT_KEY_PREFIX, trigger.area_id);

    let checkpoint_data = store.read(&key)?.context("Checkpoint not found.")?;

    let checkpoint_data: CheckpointData =
        ron::from_str(&checkpoint_data).expect("Failed to read checkpoint file.");

    // Set health
    player.1.0 = checkpoint_data.save_at_checkpoint.health;

    // Wipe invent
    player.2.0 = vec![];

    // Grant invent
    for item in &checkpoint_data.save_at_checkpoint.inventory {
        commands
            .entity(player.0)
            .give(item.item_id.clone(), item.count);
    }

    // Area
    let (area_entity, _) = areas
        .iter()
        .find(|a| a.1.0 == checkpoint_data.save_at_checkpoint.last_checkpoint_area_id)
        .expect("Unable to find area from checkpoint.");

    player.3.0 = area_entity;

    // Wind back the full path taken
    player.4.0 = checkpoint_data.save_at_checkpoint.path_taken;

    // Save the fact we've gone back
    commands.trigger(SaveRequested);

    // Trigger UI
    commands.trigger(PlayerEnteredArea(area_entity));

    info!("Checkpoint loaded!");

    Ok(())
}
