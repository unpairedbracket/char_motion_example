use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    AppSystems,
    player::{self, MovementIntent, Player},
    top_down::movement::MovementController,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(player::plugin);

    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSystems::RecordInput),
    );
}

pub fn player(meshes: &mut Assets<Mesh>, mats: &mut Assets<ColorMaterial>) -> impl Bundle {
    let mesh = Circle::new(10.0).mesh().build();
    let player_mesh = meshes.add(mesh);
    let player_color = mats.add(Color::from(tailwind::BLUE_400));

    (
        Name::new("Player"),
        Player,
        MovementController::default(),
        Mesh2d(player_mesh),
        MeshMaterial2d(player_color),
        Transform::default(),
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut intent_query: Query<&mut MovementIntent, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut intent_instance in &mut intent_query {
        intent_instance.0 = intent;
    }
}
