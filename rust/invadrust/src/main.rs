#![allow(unused)]

use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;
use crate::{
    player::{PlayerPlugin},
    components::{
        Player,
        Velocity,
        Movable,
        SpriteSize,
        Laser,
        FromPlayer,
        Enemy
    },
    enemy::{
        EnemyPlugin
    }
};

mod components;
mod player;
mod enemy;

////
const PLAYER_SPRITE: &str = "ferry.png";
const PLAYER_SIZE: (f32, f32) = (80., 54.);
const PLAYER_LASER_SPRITE: &str = "laserblue.png";
const PLAYER_LASER_SIZE: (f32, f32) = (6., 36.);

const ENEMY_SPRITE: &str = "enemy.png";
const ENEMY_SIZE: (f32, f32) = (93., 84.);
const ENEMY_LASER_SPRITE: &str = "lasered.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9., 37.);

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
//// 

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Invadrust".to_owned(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .run()
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>) {

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    let window = windows.get_primary_mut().unwrap();
    let (win_width, win_height) = (window.width(), window.height());

    let win_size = WinSize { w: win_width, h: win_height };
    commands.insert_resource(win_size); // Add resource

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE)
    };
    commands.insert_resource(game_textures); // Add resource
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>) {

    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 100.;

            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                println!("[?] DESPAWN: {:?}", entity);
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>) {
    
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // Determine collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,

                enemy_tf.translation,
                enemy_size.0 * enemy_scale
            );

            if let Some(_) = collision {
                // remove enemy
                commands.entity(enemy_entity).despawn();
                
                // remove laser
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
