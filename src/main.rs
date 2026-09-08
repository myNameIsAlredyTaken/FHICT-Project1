use bevy::prelude::*;
use std::env;

mod player;

use player::TestPlugin;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    App::new().add_plugins((DefaultPlugins, TestPlugin)).run();
}
