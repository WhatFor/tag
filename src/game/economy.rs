use crate::prelude::*;
use bevy::prelude::*;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_give_gold);
    }
}

fn on_give_gold(trigger: On<GiveGold>, mut player: Single<&mut Gold, With<Player>>) {
    player.0 += trigger.amount;
}
