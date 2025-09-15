//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    PlayMode,
    screens::Screen,
    side_scroll::{self},
    top_down::{self, GroundMaterial},
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    mut ground_mats: ResMut<Assets<GroundMaterial>>,
    mode: Res<PlayMode>,
) {
    let mut player = commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
    ));
    match *mode {
        PlayMode::SideScroll => {
            player.insert(children![side_scroll::player::player(
                &mut meshes,
                &mut mats
            )]);
        }
        PlayMode::TopDown => {
            player.insert(children![top_down::player::player(
                &mut meshes,
                &mut mats,
                &mut ground_mats
            )]);
        }
    };
}
