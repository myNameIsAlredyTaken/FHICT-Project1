use bevy::prelude::*;


const PLAYER_SPEED: f32 = 2.;
const PLAYER_SIZE: f32 = 15.;
const PLAYER_SHAPE: Circle = Circle::new(PLAYER_SIZE);
const PLAYER_COLOR: Color = Color::srgb(1., 1., 1.);



#[derive(Component, Default, Debug)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

// #[derive(Component, Debug)]
// struct Acceleration(Vec2);

#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::new(0., 0.)),
    )]
struct Player;



fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Returns Handle to asset.
    // When all references to this Handle are cleaned up, the asset is cleaned up
    let mesh = meshes.add(PLAYER_SHAPE);
    let material = materials.add(PLAYER_COLOR);

    commands.spawn((Player, Mesh2d(mesh), MeshMaterial2d(material)));
}


// Single<> skips system if none or more than 1 match is found
fn move_player(position: Single<(&mut Position, &Velocity), With<Player>>) {
    let (
        mut position,
        velocity,
        ) = position.into_inner();
    position.0 += velocity.0 * PLAYER_SPEED;
    println!("{:?}", velocity.0);
}


fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Single<&mut Velocity, With<Player>>
    ) {

        player_velocity.0 = Vec2::ZERO;

        
    if keyboard_input.pressed(KeyCode::KeyW) {
        player_velocity.0.y = PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player_velocity.0.y = -PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_velocity.0.x = -PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_velocity.0.x = PLAYER_SPEED;
    }
}


fn update_position(mut positionables: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut positionables {
        transform.translation = pos.0.extend(0.);
    }
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}


pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (
            spawn_player,
            spawn_camera
            ))
        .add_systems(FixedUpdate, (
                player_input,
                move_player,
                update_position,
                )
            .chain());
}
