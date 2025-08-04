// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

pub mod asset_tracking;
#[cfg(feature = "dev")]
pub mod dev_tools;
pub mod level;
pub mod menus;
pub mod screens;
pub mod side_scroll;
pub mod theme;
pub mod top_down;

use bevy::{asset::AssetMetaCheck, prelude::*};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn side_scroller(max_speed: f32, t_acc: f32, a_stop: f32, a_rev: f32) {
    run_side_scroll(max_speed, t_acc, a_stop, a_rev);
}

#[wasm_bindgen]
pub fn top_down(max_speed: f32, t_acc: f32, a_stop: f32, a_rev: f32) {
    run_top_down(max_speed, t_acc, a_stop, a_rev);
}

pub fn run_side_scroll(max_speed: f32, t_acc: f32, a_stop: f32, a_rev: f32) -> AppExit {
    App::new()
        .add_plugins(AppPlugin {
            mode: PlayMode::SideScroll,
            params: MotionParameters::full(max_speed, t_acc, a_stop, a_rev),
        })
        .run()
}

pub fn run_side_scroll_basic() -> AppExit {
    App::new()
        .add_plugins(AppPlugin {
            mode: PlayMode::SideScroll,
            params: MotionParameters::basic(600.0, 1.0),
        })
        .run()
}

pub fn run_side_scroll_stop() -> AppExit {
    App::new()
        .add_plugins(AppPlugin {
            mode: PlayMode::SideScroll,
            params: MotionParameters::with_stopping(600.0, 1.0, 5.0),
        })
        .run()
}

pub fn run_side_scroll_reverse() -> AppExit {
    App::new()
        .add_plugins(AppPlugin {
            mode: PlayMode::SideScroll,
            params: MotionParameters::full(600.0, 1.0, 5.0, 5.0),
        })
        .run()
}

pub fn run_top_down(max_speed: f32, t_acc: f32, a_stop: f32, a_rev: f32) -> AppExit {
    App::new()
        .add_plugins(AppPlugin {
            mode: PlayMode::TopDown,
            params: MotionParameters::basic(600.0, 1.0),
        })
        .run()
}

#[derive(Clone, Copy, Resource)]
pub enum PlayMode {
    TopDown,
    SideScroll,
}

#[derive(Clone, Copy, Resource)]
pub struct MotionParameters {
    max_speed: f32,
    alpha_rev: f32,
    alpha_stop: f32,
    t_acc: f32,
}

impl MotionParameters {
    pub fn basic(max_speed: f32, t_acc: f32) -> Self {
        Self {
            max_speed,
            t_acc,
            alpha_rev: 1.0,
            alpha_stop: 1.0,
        }
    }

    pub fn with_stopping(max_speed: f32, t_acc: f32, alpha_stop: f32) -> Self {
        Self {
            max_speed,
            t_acc,
            alpha_rev: 1.0,
            alpha_stop,
        }
    }

    pub fn full(max_speed: f32, t_acc: f32, alpha_stop: f32, alpha_rev: f32) -> Self {
        Self {
            max_speed,
            t_acc,
            alpha_rev,
            alpha_stop,
        }
    }
}

impl std::fmt::Display for PlayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayMode::TopDown => write!(f, "Top Down"),
            PlayMode::SideScroll => write!(f, "Side Scroll"),
        }
    }
}

pub struct AppPlugin {
    mode: PlayMode,
    params: MotionParameters,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "character motion example".into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        match self.mode {
            PlayMode::TopDown => app.add_plugins(top_down::plugin),
            PlayMode::SideScroll => app.add_plugins(side_scroll::plugin),
        };

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        app.insert_resource(self.mode);
        app.insert_resource(self.params);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
