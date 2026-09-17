use bevy:: prelude::*;
use crate::animation_controller::{
    AnimationController, Direction, State, SpriteSheet
};


const PLAYER_SPEED: f32 = 100.;


#[derive(Component, Default, Debug)]
#[require(Transform)]
struct Position(Vec2);


#[derive(Component, Default)]
struct Velocity(Vec2);


#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::new(0., 0.)),
    )]
pub(super) struct Player;



fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_atlas: Res<SpriteSheet>,
) {
    // Returns Handle to asset.
    // When all references to this Handle are cleaned up, the asset is cleaned up
    let image = asset_server
        .load_builder()
        .override_unapproved()
        .load("Sprites\\Char\\char_a_p1_0bas_humn_v00.png");

    commands.spawn((
        AnimationController::default(),
        Player,
        Sprite {
        image: image.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: sprite_atlas.0.clone(),
            index: 40
        }),
        ..default()
    }));
}


// Single<> skips system if none or more than 1 match is found
fn move_player(position: Single<(&mut Position, &Velocity),
    With<Player>>,
    time: Res<Time>
) {
    let (mut position, velocity) = position.into_inner();
    position.0 += velocity.0 * PLAYER_SPEED * time.delta_secs();
}


fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<( &mut AnimationController, &mut Velocity), With<Player>>,
) {
    let (
        mut anim_control,
        mut player_velocity
    ) = query.single_mut().unwrap();

    player_velocity.0 = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        player_velocity.0.y = PLAYER_SPEED;
        anim_control.change_state(State::Walk);
        anim_control.change_dir(Direction::Up);
    }
    
    if keyboard_input.pressed(KeyCode::KeyS) {
        player_velocity.0.y = -PLAYER_SPEED;
        anim_control.change_state(State::Walk);
        anim_control.change_dir(Direction::Down);

    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_velocity.0.x = -PLAYER_SPEED;
        anim_control.change_state(State::Walk);
        anim_control.change_dir(Direction::Left);
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        player_velocity.0.x = PLAYER_SPEED;
        anim_control.change_state(State::Walk);
        anim_control.change_dir(Direction::Right);
    }

    if !keyboard_input.any_pressed([
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
    ]){
        anim_control.change_state(State::Idle);
    }

    player_velocity.0 = player_velocity.0.normalize_or_zero();
}


fn update_position(mut positionables: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut positionables {
        transform.translation = pos.0.extend(0.);
        transform.translation.z = -transform.translation.y * 0.1;
    }
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}


pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_player, spawn_camera))
        .add_systems(
            FixedUpdate,
            (player_input, move_player, update_position).chain(),
        );
    app.init_resource::<SpriteSheet>();
}
