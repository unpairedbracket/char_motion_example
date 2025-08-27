use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<ScreenWrap>();

    app.add_systems(
        Update,
        apply_screen_wrap
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(ScreenWrap)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementIntent(pub Vec2);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ScreenWrap;

pub fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<&Projection, With<Camera2d>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = match *camera {
        Projection::Orthographic(proj) => proj.area.size(),
        _ => window.size(),
    };
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
