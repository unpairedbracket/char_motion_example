use bevy::prelude::*;

use crate::{
    AppSystems, MotionParameters, player::MovementIntent, top_down::level::GroundRotation,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();

    app.add_systems(Update, apply_movement.in_set(AppSystems::Update));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(MovementIntent, GroundRotation)]
pub struct MovementController {
    velocity: Vec2,
}

pub(super) fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&mut MovementController, &MovementIntent, &mut Transform)>,
    params: Res<MotionParameters>,
) {
    for (mut controller, intent, mut transform) in &mut movement_query {
        let scaled_timestep = time.delta_secs() / params.t_acc;
        if let Some(intent_direction) = intent.0.try_normalize() {
            let target_velocity = params.max_speed * intent.0;
            let longitudinal_speed = intent_direction.dot(controller.velocity);
            let longitudinal_velocity = intent_direction * longitudinal_speed;
            let transverse_velocity = controller.velocity - longitudinal_velocity;
            let alpha_longitudinal = match longitudinal_speed {
                vel if vel < 0.0 => params.alpha_rev,
                0.0 => params.alpha_stop,
                vel if vel > 0.0 => 1.0,
                _ => 1.0,
            };
            let new_long_velocity = (longitudinal_velocity + scaled_timestep * target_velocity)
                / (1.0 + alpha_longitudinal * scaled_timestep);

            let new_trans_velocity =
                transverse_velocity / (1.0 + params.alpha_turn * scaled_timestep);
            controller.velocity = new_long_velocity + new_trans_velocity;
        } else {
            controller.velocity /= 1.0 + params.alpha_stop * scaled_timestep;
        }

        transform.translation += controller.velocity.extend(0.0) * time.delta_secs();
    }
}
