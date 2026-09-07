use bevy::{ecs::query, prelude::*, state::commands};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (test, greet).chain());
    }
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Test".to_string())));
}
