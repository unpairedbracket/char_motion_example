use bevy::{color::palettes::tailwind, prelude::*, render::camera::ScalingMode};

use crate::{
    AppSystems,
    player::{self, MovementIntent, Player, TrackingCameras},
    top_down::{level::GroundMaterial, movement::MovementController},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(player::plugin);

    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSystems::RecordInput),
    );
}

pub fn player(
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<ColorMaterial>,
    ground_mat: &mut Assets<GroundMaterial>,
) -> impl Bundle {
    let mesh = Circle::new(10.0).mesh().build();
    let player_mesh = meshes.add(mesh);
    let player_colour = mats.add(Color::from(tailwind::BLUE_400));

    let bg_mesh = meshes.add(Rectangle::new(1000.0, 1000.0).mesh().build());
    let bg_mat = ground_mat.add(tailwind::RED_500);

    (
        Name::new("Player"),
        Player,
        MovementController::default(),
        Mesh2d(player_mesh),
        MeshMaterial2d(player_colour),
        Transform::default(),
        related!(
            TrackingCameras[(
                Name::new("Camera"),
                Camera2d,
                Projection::Orthographic(OrthographicProjection {
                    scaling_mode: ScalingMode::FixedHorizontal {
                        viewport_width: 1000.0,
                    },
                    ..OrthographicProjection::default_2d()
                }),
                children![(
                    Transform::from_xyz(0.0, 0.0, -10.0),
                    Mesh2d(bg_mesh),
                    MeshMaterial2d(bg_mat)
                )]
            )]
        ),
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut intent_query: Query<&mut MovementIntent, With<Player>>,
) {
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

    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut intent_instance in &mut intent_query {
        intent_instance.0 = intent;
    }
}
