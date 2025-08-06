use bevy::prelude::*;

use crate::{AppSystems, MotionParameters, PausableSystems, side_scroll::movement::MovementIntent};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BasicMovementController>();

    app.add_systems(
        Update,
        apply_movement
            .before(super::apply_screen_wrap)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(MovementIntent)]
pub struct BasicMovementController {
    pub velocity: f32,
}

impl Default for BasicMovementController {
    fn default() -> Self {
        Self { velocity: 0.0 }
    }
}

pub(super) fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(
        &mut BasicMovementController,
        &MovementIntent,
        &mut Transform,
    )>,
    params: Res<MotionParameters>,
) {
    for (mut controller, intent, mut transform) in &mut movement_query {
        let target_velocity = params.max_speed * intent.0.x;
        let scaled_timestep = time.delta_secs() / params.t_acc;
        let alpha = match target_velocity * controller.velocity.signum() {
            vel if vel < 0.0 => params.alpha_rev,
            vel if vel == 0.0 => params.alpha_stop,
            vel if vel > 0.0 => 1.0,
            _ => 1.0,
        };
        controller.velocity = (controller.velocity + scaled_timestep * target_velocity)
            / (1.0 + alpha * scaled_timestep);

        transform.translation += controller.velocity * Vec3::X * time.delta_secs();
    }
}
