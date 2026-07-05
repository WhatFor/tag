use crate::prelude::*;
use bevy::prelude::*;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::InCombat),
            start_combat.in_set(PausableSystems),
        );
    }
}

fn start_combat(
    mut commands: Commands,
    area: Single<&CurrentArea, With<Player>>,
    all_area_content: Query<&AreaContent, With<Area>>,
) {
    let Ok(content) = all_area_content.get(area.0) else {
        return;
    };

    if let AreaContent::Combat {
        start_lines,
        enemy_ids,
        win_lines,
        lose_lines,
    } = content
    {
        for line in start_lines {
            info!(line);
        }
    } else {
        warn!("Player is in combat state but not in a combat area!");
    }
}
