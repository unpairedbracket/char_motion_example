use bevy::prelude::*;

use crate::{
    AppSystems, MotionParameters, player::MovementIntent, side_scroll::level::PositionAlongGround,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BasicMovementController>();

    app.add_systems(Update, apply_movement.in_set(AppSystems::Update));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(MovementIntent)]
pub struct BasicMovementController {
    velocity: f32,
}

pub(super) fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(
        &mut BasicMovementController,
        &MovementIntent,
        &mut PositionAlongGround,
        &Transform,
    )>,
    params: Res<MotionParameters>,
) {
    let gravity_global = 50. * params.gravity_strength * Vec3::NEG_Y;
    for (mut controller, intent, mut arc_position, tf) in &mut movement_query {
        let gravity_local = tf.rotation.inverse() * gravity_global;

        let a_max = params.max_speed / params.t_acc * intent.0.x.signum();
        let g_over_a = gravity_local.x / a_max;
        let slope_factor = (1.0 + g_over_a * g_over_a).sqrt() + g_over_a;

        let target_velocity = slope_factor * params.max_speed * intent.0.x;
        let scaled_timestep = time.delta_secs() / params.t_acc;
        let alpha = match target_velocity * controller.velocity.signum() {
            vel if vel < 0.0 => params.alpha_rev,
            0.0 => params.alpha_stop,
            vel if vel > 0.0 => 1.0,
            _ => 1.0,
        };
        controller.velocity = (controller.velocity + scaled_timestep * target_velocity)
            / (1.0 + alpha * scaled_timestep);

        arc_position.0 += controller.velocity * time.delta_secs();
    }
}
