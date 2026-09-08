use bevy::{ecs::query, prelude::*, state::commands};

pub struct TestPlugin;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Test".to_string())));
}

fn test() {
    println!("123");
}

fn greet(query: Query<&Name, With<Person>>) {
    for name in query {
        println!("test {}", name.0);
    }
}

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (test, greet).chain());
    }
}
