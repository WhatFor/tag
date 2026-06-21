use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum AudioChannel {
    Ambience,
    Sfx,
    Music,
}
