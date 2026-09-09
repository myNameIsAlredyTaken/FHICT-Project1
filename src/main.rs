use bevy::prelude::*;

mod enemy;
mod game;
mod player;

fn main() {
    // unsafe {
    //     env::set_var("RUST_BACKTRACE", "1");
    // }
    App::new()
        .add_plugins((DefaultPlugins, player::plugin))
        .run();
}
