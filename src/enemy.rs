use bevy::prelude::*;
use crate::animation_controller::{
    AnimationController, Direction, State, SpriteSheet
};
use crate::player::Player;

const ENEMY_SPEED: f32 = 100.;

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
struct Enemy;


// fn update_position(mut positionables: Query<(&Position, &mut Transform)>) {
//     for (pos, mut transform) in &mut positionables {
//         transform.translation = pos.0.extend(0.);
//         transform.translation.z = -transform.translation.y * 0.1;
//     }
// }

fn fetch_player_pos(
    player: Query<&Position, With<Player>>,
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>
) {
    let player_pos = player.single_inner().unwrap();

    for mut enemy_transform in &mut enemies {
        
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
            index: 40
        }),
        ..default()
    }));
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemy);
    app.add_systems(Update, fetch_player_pos);
}
