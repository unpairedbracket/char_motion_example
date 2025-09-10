use std::f32::consts::PI;

use bevy::{color::palettes::tailwind, prelude::*, render::camera::ScalingMode};

use crate::{
    player::{Player, TrackingCameras},
    top_down::movement,
};

pub fn plugin(app: &mut App) {
    app.register_type::<Ground>();

    app.add_systems(
        Update,
        (swap_ground, move_along_ground, move_camera)
            .chain()
            .after(movement::apply_movement),
    );
}

#[derive(Resource, Reflect)]
pub enum Ground {
    FlatPeriodic,
    Hills,
}

fn swap_ground(mut ground: ResMut<Ground>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyC) {
        *ground = match *ground {
            Ground::FlatPeriodic => Ground::Hills,
            Ground::Hills => Ground::FlatPeriodic,
        }
    }
}

pub fn move_along_ground(mut objects: Query<&mut Transform>, ground: Res<Ground>) {
    match *ground {
        Ground::FlatPeriodic => {}
        Ground::Hills => {
            for mut tf in &mut objects {
                let kx = 2.0 * PI / 500.0;
                let ky = 2.0 * PI / 2000.0;

                let h = (kx * tf.translation.x).cos() * (ky * tf.translation.y).cos();
                tf.translation.z = h;

                let dz_dx = kx * (kx * tf.translation.x).sin() * (ky * tf.translation.y).cos();
                let dz_dy = ky * (ky * tf.translation.y).sin() * (kx * tf.translation.x).cos();
            }
        }
    }
}

pub fn set_proj(mut cameras: Query<&mut Projection>) {
    for mut proj in &mut cameras {
        match &mut *proj {
            Projection::Orthographic(proj) => {
                warn!("{:?}", proj.scaling_mode);
                proj.scaling_mode = ScalingMode::FixedHorizontal {
                    viewport_width: 1000.0,
                };
            }
            _ => {}
        }
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
    }
}

// pub fn draw_ground(
//     players: Query<(&Transform), With<Player>>,
//     ground: Res<Ground>,
//     mut gizmo: Gizmos,
// ) {
//     match *ground {
//         Ground::FlatPeriodic => {
//             for (tform, _) in &players {
//                 let tf2 = tform.translation.xy();
//                 let draw_distance = 500.0;
//                 gizmo.line_2d(
//                     tf2 - draw_distance * Vec2::X,
//                     tf2 + draw_distance * Vec2::X,
//                     tailwind::BLUE_300,
//                 );
//             }
//         }
//         Ground::Hills => {
//             let period_length: f32 = 1000.0;
//             let max_height = 500.0;
//             let h_squared = max_height * max_height;
//             let period_arclength = (period_length * period_length + 4.0 * h_squared).sqrt();
//             let excess = 0.5 * (period_arclength - period_length);
//             let max_period = 4.0;
//             for (_, arc_pos) in &players {
//                 let period = arc_pos.0.div_euclid(period_arclength);
//                 for draw_period in [period - 2., period - 1., period, period + 1., period + 2.] {
//                     let period_folded = ((draw_period + max_period).rem_euclid(2.0 * max_period)
//                         - max_period)
//                         .abs();
//                     let flat_fraction = period_folded / (max_period + 1.0);
//                     // Flat length within the period is 25% at 0, 50% at top, 25% at zero
//                     let total_flat_length = period_length * flat_fraction;

//                     let current_max_height = (h_squared - total_flat_length * excess).sqrt();

//                     let period_start = period_length * draw_period;

//                     let slope_start = total_flat_length / 4.0;
//                     let slope_end = period_length / 2.0 - slope_start;

//                     let downslope_start = period_length / 2.0 + slope_start;
//                     let downslope_end = period_length - slope_start;

//                     let start_point = Vec2::new(period_start, 0.0);
//                     let points = [
//                         start_point,
//                         start_point + slope_start * Vec2::X,
//                         start_point + slope_end * Vec2::X + current_max_height * Vec2::Y,
//                         start_point + downslope_start * Vec2::X + current_max_height * Vec2::Y,
//                         start_point + downslope_end * Vec2::X,
//                         start_point + period_length * Vec2::X,
//                     ];

//                     gizmo.linestrip_2d(points, tailwind::BLUE_300);
//                 }
//             }
//         }
//         Ground::Loops => todo!(),
//     }
// }
