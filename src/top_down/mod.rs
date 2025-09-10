use bevy::prelude::*;

mod level;
mod movement;
pub mod player;

pub use level::Ground;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, player::plugin, level::plugin));
}
