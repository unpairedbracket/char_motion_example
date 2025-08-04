//! Player-specific behavior.

use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    top_down::movement::{MovementController, ScreenWrap},
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
pub fn player(
    max_speed: f32,
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    let mesh = Circle::new(10.0).mesh().build();
    let player_mesh = meshes.add(mesh);
    let player_color = mats.add(Color::from(tailwind::BLUE_400));

    (
        Name::new("Player"),
        Player,
        Mesh2d(player_mesh),
        MeshMaterial2d(player_color),
        Transform::default(),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
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
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}
