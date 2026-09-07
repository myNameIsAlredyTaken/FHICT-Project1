use bevy::prelude::*;

mod player;

use player::TestPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, TestPlugin)).run();
}
