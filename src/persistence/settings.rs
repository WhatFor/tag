use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::data::SettingsData;
use crate::persistence::events::SettingsUpdated;
use crate::persistence::resources::SaveBackend;
use crate::persistence::{SETTINGS_FILE_KEY, SETTINGS_FORMAT_VERSION};

pub struct PersistenceSettingsPlugin;

impl Plugin for PersistenceSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_requested);
    }
}

fn on_save_requested(
    _: On<SettingsUpdated>,
    store: Res<SaveBackend>,
    settings: Res<AudioSettings>,
) -> Result {
    info!("Saving settings...");

    let save_data = SettingsData {
        version: SETTINGS_FORMAT_VERSION,
        audio: settings.clone(),
    };

    let save_data = ron::to_string(&save_data)?;

    if let Err(error) = store.write(SETTINGS_FILE_KEY, &save_data) {
        warn!("Save settings failed: {}", error);
    } else {
        info!("Settings saved!");
    }

    Ok(())
}
