use std::f32::consts::PI;

use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    player::{Player, TrackingCameras},
    top_down::movement,
};

pub fn plugin(app: &mut App) {
    app.register_type::<Ground>();

    app.add_systems(
        Update,
        (swap_ground, move_along_ground, move_camera, draw_ground)
            .chain()
            .after(movement::apply_movement),
    );
}

#[derive(Component, Reflect, Default)]
pub struct GroundRotation(Quat);

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

pub fn move_along_ground(
    mut objects: Query<(&mut Transform, &mut GroundRotation)>,
    ground: Res<Ground>,
) {
    match *ground {
        Ground::FlatPeriodic => {}
        Ground::Hills => {
            for (mut tf, mut gr) in &mut objects {
                let kx = 2.0 * PI / 1000.0;
                let ky = 2.0 * PI / 500.0;
                let h0 = 100.0;

                let h = h0 * (kx * tf.translation.x).cos() * (ky * tf.translation.y).cos();
                tf.translation.z = h;

                let dz_dx = kx * h0 * (kx * tf.translation.x).sin() * (ky * tf.translation.y).cos();
                let dz_dy = ky * h0 * (ky * tf.translation.y).sin() * (kx * tf.translation.x).cos();
                // let dz_dx = 0.;
                // let dz_dy = (kx * tf.translation.x).sin();

                let normal = Dir3::from_xyz(-dz_dx, -dz_dy, 1.0).unwrap();
                gr.0 = Quat::from_rotation_arc(Vec3::Z, *normal);
            }
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

pub fn draw_ground(
    players: Query<(&Transform, &GroundRotation), With<Player>>,
    ground: Res<Ground>,
    mut gizmo: Gizmos,
) {
    match *ground {
        Ground::FlatPeriodic => {}
        Ground::Hills => {
            for (tform, gr) in &players {
                let pos = tform.translation;
                let norm = gr.0 * Vec3::Z;

                gizmo.arrow(pos, pos + 100.0 * norm, tailwind::BLUE_300);
            }
        }
    }
}
