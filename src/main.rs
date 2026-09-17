use bevy::prelude::*;

mod debug; mod enemy;
mod animation_controller;
mod player;

fn main() {
    // unsafe {
    //     env::set_var("RUST_BACKTRACE", "1");
    // }
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow,
                ..default()
            }),
            // debug::plugin,
            player::plugin,
            animation_controller::plugin,
            enemy::plugin
        ))
        .run();
}
