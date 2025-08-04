//! Player-specific behavior.

use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    side_scroll::movement::{MovementIntent, ScreenWrap, basic::BasicMovementController},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(meshes: &mut Assets<Mesh>, mats: &mut Assets<ColorMaterial>) -> impl Bundle {
    let mesh = Capsule2d::new(10.0, 30.0).mesh().build();
    let player_mesh = meshes.add(mesh);
    let player_color = mats.add(Color::from(tailwind::BLUE_400));

    (
        Name::new("Player"),
        Player,
        Mesh2d(player_mesh),
        MeshMaterial2d(player_color),
        Transform::default(),
        ScreenWrap,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(BasicMovementController)]
struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut intent_query: Query<&mut MovementIntent, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;

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
