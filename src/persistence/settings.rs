use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::data::SettingsData;
use crate::persistence::events::SettingsUpdated;
use crate::persistence::resources::SaveBackend;
use crate::persistence::{SETTINGS_FILE_KEY, SETTINGS_FORMAT_VERSION};

pub struct PersistenceSettingsPlugin;

impl Plugin for PersistenceSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initialising), load_settings);
        app.add_observer(on_save_requested);
    }
}

fn load_settings(store: Res<SaveBackend>, mut settings: ResMut<AudioSettings>) -> Result {
    let save_data = store.read(SETTINGS_FILE_KEY);

    let start_data = match save_data {
        Ok(Some(save_data)) => {
            info!(save_data);

            let data: SettingsData =
                ron::from_str(&save_data).expect("Failed to read settings file.");

            data
        }
        _ => {
            info!("No settings data!");

            SettingsData {
                version: SETTINGS_FORMAT_VERSION,
                audio: AudioSettings::default(),
            }
        }
    };

    if start_data.version != SETTINGS_FORMAT_VERSION {
        warn!("Settings data out of date!");
        return Ok(());
    }

    settings.master_volume = start_data.audio.master_volume;
    settings.music_volume = start_data.audio.music_volume;
    settings.sfx_volume = start_data.audio.sfx_volume;
    settings.ambience_volume = start_data.audio.ambience_volume;

    Ok(())
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
