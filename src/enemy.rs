use crate::animation_controller::{AnimationController, Direction, SpriteSheet, State};
use crate::player::Player;
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 80.;

#[derive(Component, Default, Debug)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Debug)]
struct Dir(Vec2);

#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::new(0., 0.)),
    Dir = Dir(Vec2::new(0., 0.)),
    )]
struct Enemy;

// fn update_position(mut positionables: Query<(&Position, &mut Transform)>) {
//     for (pos, mut transform) in &mut positionables {
//         transform.translation = pos.0.extend(0.);
//         transform.translation.z = -transform.translation.y * 0.1;
//     }
// }

fn check_dir(
    mut query: Query<
        (
            &mut AnimationController,
            &mut Position,
            &mut Transform,
            &mut Velocity,
        ),
        With<Enemy>,
    >,
) {
    let (mut anim_control,
        enemy_pos,
        enemy_transform,
        mut enemy_velocity
    ) = query.single_mut().unwrap();

    let mut enemy_pos = enemy_pos.0;
    let enemy_transform = enemy_transform.translation.truncate();


    let diff = enemy_pos - enemy_transform;
    println!("enemy_pos: {:?}", enemy_pos);
    println!("enemy_transform: {:?}", enemy_transform);


    let diff = diff.normalize().round();

    if diff.x > 0. {
        anim_control.change_dir(Direction::Left);
        anim_control.change_state(State::Walk);
        enemy_velocity.0.x = ENEMY_SPEED;
    } else if diff.x < 0. {
        anim_control.change_dir(Direction::Right);
        anim_control.change_state(State::Walk);
        enemy_velocity.0.x = ENEMY_SPEED;
    } else if diff.y > 0. {
        anim_control.change_dir(Direction::Down);
        anim_control.change_state(State::Walk);
        enemy_velocity.0.x = ENEMY_SPEED;
    } else {
        anim_control.change_dir(Direction::Up);
        anim_control.change_state(State::Walk);
        enemy_velocity.0.x = ENEMY_SPEED;
    }

    // println!("{:?}", diff);

    enemy_pos = enemy_transform;
}

fn fetch_player_pos(
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    position: Single<(&mut Position, &mut Dir), (With<Enemy>, Without<Player>)>,
) {
    let player_pos = player.single_inner().unwrap();
    let (mut position, mut direction) = position.into_inner();

    for mut enemy_transform in &mut enemies {
        direction.0 = (player_pos.translation - enemy_transform.translation)
            .truncate()
            .normalize_or_zero();
        println!("{:?}", direction.0);


        position.0 = enemy_transform.translation.truncate();

        enemy_transform.translation.z = -enemy_transform.translation.y * 0.1;
    }
}

fn move_enemy(mut position: Query<(&mut Position, &Dir), With<Enemy>>, time: Res<Time>) {
    let (mut position, direction) = position.single_mut().unwrap();
    position.0 += direction.0 * ENEMY_SPEED * time.delta_secs();
}

fn update_position(mut positionables: Query<(&Position, &mut Transform), With<Enemy>>) {
    for (pos, mut transform) in &mut positionables {
        transform.translation = pos.0.extend(0.);
        transform.translation.z = -transform.translation.y * 0.1;
    }
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_atlas: Res<SpriteSheet>,
) {
    // Returns Handle to asset.
    // When all references to this Handle are cleaned up, the asset is cleaned up
    let image = asset_server
        .load_builder()
        .override_unapproved()
        .load("Sprites\\Char\\enemy.png");

    commands.spawn((
        AnimationController::default(),
        Enemy,
        Sprite {
            image: image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sprite_atlas.0.clone(),
                index: 40,
            }),
            ..default()
        },
    ));
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemy);
    app.add_systems(
        Update,
        (fetch_player_pos, check_dir, move_enemy, update_position).chain(),
    );
}
