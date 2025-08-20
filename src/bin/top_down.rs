// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

pub fn main() -> AppExit {
    char_motion_example::run_top_down(500.0, 1.0, 5.0, 5.0, 5.0)
}
