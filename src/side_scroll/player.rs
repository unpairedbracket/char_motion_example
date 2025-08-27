use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    player::{self, MovementIntent, Player},
    side_scroll::movement::BasicMovementController,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(player::plugin);

    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

pub fn player(meshes: &mut Assets<Mesh>, mats: &mut Assets<ColorMaterial>) -> impl Bundle {
    let mesh = Capsule2d::new(10.0, 30.0).mesh().build();
    let player_mesh = meshes.add(mesh);
    let player_color = mats.add(Color::from(tailwind::BLUE_400));

    (
        Name::new("Player"),
        Player,
        BasicMovementController::default(),
        Mesh2d(player_mesh),
        MeshMaterial2d(player_color),
        Transform::default(),
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut intent_query: Query<&mut MovementIntent, With<Player>>,
) {
    let mut intent = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    let intent = intent.normalize_or_zero();

    for mut intent_instance in &mut intent_query {
        intent_instance.0 = intent;
    }
}
