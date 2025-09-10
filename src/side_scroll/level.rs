use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    player::{Player, TrackingCameras},
    side_scroll::movement,
};

pub fn plugin(app: &mut App) {
    app.register_type::<Ground>();
    app.register_type::<PositionAlongGround>();

    app.add_systems(
        Update,
        (swap_ground, move_along_ground, move_camera, draw_ground)
            .chain()
            .after(movement::apply_movement),
    );
}

#[derive(Resource, Reflect)]
pub enum Ground {
    FlatPeriodic,
    Hills,
    Loops,
}

fn swap_ground(mut ground: ResMut<Ground>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyC) {
        *ground = match *ground {
            Ground::FlatPeriodic => Ground::Hills,
            Ground::Hills => Ground::FlatPeriodic,
            Ground::Loops => Ground::Loops,
        }
    }
}

#[derive(Component, Reflect)]
pub struct PositionAlongGround(pub f32);

pub fn move_along_ground(
    mut objects: Query<(&mut Transform, &PositionAlongGround)>,
    ground: Res<Ground>,
) {
    match *ground {
        Ground::FlatPeriodic => {
            for (mut tform, arc_pos) in &mut objects {
                tform.translation = Vec3::new(arc_pos.0, 0., 0.);
            }
        }
        Ground::Hills => {
            let period_length: f32 = 1000.0;
            let max_height = 500.0;
            let h_squared = max_height * max_height;
            let period_arclength = (period_length * period_length + 4.0 * h_squared).sqrt();
            let excess = 0.5 * (period_arclength - period_length);
            let max_period = 4.0;
            for (mut tform, arc_pos) in &mut objects {
                let arc_pos_per_period = arc_pos.0 / period_arclength;
                let period = arc_pos.0.div_euclid(period_arclength);

                let period_folded =
                    ((period + max_period).rem_euclid(2.0 * max_period) - max_period).abs();
                let flat_fraction = period_folded / (max_period + 1.0);
                // Flat length within the period is 25% at 0, 50% at top, 25% at zero
                let total_flat_length = period_length * flat_fraction;

                let current_max_height = (h_squared - total_flat_length * excess).sqrt();

                let fractional_arc_pos = arc_pos_per_period.rem_euclid(1.0);
                let symmetric_fractional_arc_pos = 0.5 - (fractional_arc_pos - 0.5).abs();
                let (x_pos_in_period, y_pos, angle) = if symmetric_fractional_arc_pos
                    * period_arclength
                    < flat_fraction * 0.25 * period_length
                {
                    (symmetric_fractional_arc_pos * period_arclength, 0.0, 0.0)
                } else if (0.5 - symmetric_fractional_arc_pos) * period_arclength
                    < flat_fraction * 0.25 * period_length
                {
                    (
                        0.5 * period_length
                            - (0.5 - symmetric_fractional_arc_pos) * period_arclength,
                        current_max_height,
                        0.0,
                    )
                } else {
                    let t = (2.0 * symmetric_fractional_arc_pos * period_arclength
                        - (0.5 * flat_fraction * period_length))
                        / (period_arclength - flat_fraction * period_length);

                    (
                        (0.25 * flat_fraction * period_length)
                            + (0.5 * period_length - 0.5 * flat_fraction * period_length) * t,
                        t * current_max_height,
                        current_max_height.atan2((1.0 - flat_fraction) * period_length * 0.5),
                    )
                };

                let (x_pos_in_period, angle) = if fractional_arc_pos > 0.5 {
                    (period_length - x_pos_in_period, -angle)
                } else {
                    (x_pos_in_period, angle)
                };

                let x_pos = period * period_length + x_pos_in_period;

                tform.translation = Vec3::new(x_pos, y_pos, 0.);
                tform.rotation = Quat::from_rotation_z(angle);
            }
        }
        Ground::Loops => todo!(),
    }
}

pub fn move_camera(
    mut cameras: Query<(&mut Transform, &Projection), Without<Player>>,
    players: Query<(&Transform, &TrackingCameras), With<Player>>,
    ground: Res<Ground>,
) {
    match *ground {
        Ground::FlatPeriodic => {
            for (player_transform, its_cameras) in &players {
                for camera in its_cameras.iter() {
                    if let Ok((mut camera_transform, Projection::Orthographic(proj))) =
                        cameras.get_mut(camera)
                    {
                        let camera_width = proj.area.width();

                        if player_transform.translation.x
                            > camera_transform.translation.x + camera_width / 2.0
                        {
                            camera_transform.translation.x += camera_width;
                        } else if player_transform.translation.x
                            < camera_transform.translation.x - camera_width / 2.0
                        {
                            camera_transform.translation.x -= camera_width;
                        }

                        let camera_height = proj.area.height();
                        if player_transform.translation.y
                            > camera_transform.translation.y + camera_height / 2.0
                        {
                            camera_transform.translation.y += camera_height;
                        } else if player_transform.translation.y
                            < camera_transform.translation.y - camera_height / 2.0
                        {
                            camera_transform.translation.y -= camera_height;
                        }
                    }
                }
            }
        }
        Ground::Hills => {
            for (player_transform, its_cameras) in &players {
                for camera in its_cameras.iter() {
                    if let Ok((mut camera_transform, Projection::Orthographic(proj))) =
                        cameras.get_mut(camera)
                    {
                        let cam_area = proj.area.inflate(-100.);
                        let Vec2 {
                            x: camera_width,
                            y: camera_height,
                        } = cam_area.half_size();
                        if player_transform.translation.x
                            > camera_transform.translation.x + camera_width
                        {
                            camera_transform.translation.x =
                                player_transform.translation.x - camera_width;
                        } else if player_transform.translation.x
                            < camera_transform.translation.x - camera_width
                        {
                            camera_transform.translation.x =
                                player_transform.translation.x + camera_width;
                        }

                        if player_transform.translation.y
                            > camera_transform.translation.y + camera_height
                        {
                            camera_transform.translation.y =
                                player_transform.translation.y - camera_height;
                        } else if player_transform.translation.y
                            < camera_transform.translation.y - camera_height
                        {
                            camera_transform.translation.y =
                                player_transform.translation.y + camera_height;
                        }
                    }
                }
            }
        }
        Ground::Loops => todo!(),
    }
}

pub fn draw_ground(
    players: Query<(&Transform, &PositionAlongGround), With<Player>>,
    ground: Res<Ground>,
    mut gizmo: Gizmos,
) {
    match *ground {
        Ground::FlatPeriodic => {
            for (tform, _) in &players {
                let tf2 = tform.translation.xy();
                let draw_distance = 500.0;
                gizmo.line_2d(
                    tf2 - draw_distance * Vec2::X,
                    tf2 + draw_distance * Vec2::X,
                    tailwind::BLUE_300,
                );
            }
        }
        Ground::Hills => {
            let period_length: f32 = 1000.0;
            let max_height = 500.0;
            let h_squared = max_height * max_height;
            let period_arclength = (period_length * period_length + 4.0 * h_squared).sqrt();
            let excess = 0.5 * (period_arclength - period_length);
            let max_period = 4.0;
            for (_, arc_pos) in &players {
                let period = arc_pos.0.div_euclid(period_arclength);
                for draw_period in [period - 2., period - 1., period, period + 1., period + 2.] {
                    let period_folded = ((draw_period + max_period).rem_euclid(2.0 * max_period)
                        - max_period)
                        .abs();
                    let flat_fraction = period_folded / (max_period + 1.0);
                    // Flat length within the period is 25% at 0, 50% at top, 25% at zero
                    let total_flat_length = period_length * flat_fraction;

                    let current_max_height = (h_squared - total_flat_length * excess).sqrt();

                    let period_start = period_length * draw_period;

                    let slope_start = total_flat_length / 4.0;
                    let slope_end = period_length / 2.0 - slope_start;

                    let downslope_start = period_length / 2.0 + slope_start;
                    let downslope_end = period_length - slope_start;

                    let start_point = Vec2::new(period_start, 0.0);
                    let points = [
                        start_point,
                        start_point + slope_start * Vec2::X,
                        start_point + slope_end * Vec2::X + current_max_height * Vec2::Y,
                        start_point + downslope_start * Vec2::X + current_max_height * Vec2::Y,
                        start_point + downslope_end * Vec2::X,
                        start_point + period_length * Vec2::X,
                    ];

                    gizmo.linestrip_2d(points, tailwind::BLUE_300);
                }
            }
        }
        Ground::Loops => todo!(),
    }
}
