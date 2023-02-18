#![allow(unused)]

use bevy::prelude::*;
use crate::player::{PlayerPlugin};

mod components;
mod player;


const PLAYER_SPRITE: &str = "ferry.png";
const PLAYER_SIZE: (f32, f32) = (80., 54.);
const PLAYER_LASER_SPRITE: &str = "laserblue.png";
const PLAYER_LASER_SIZE: (f32, f32) = (6., 36.);

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
 

pub struct WinSize {
    pub w: f32,
    pub h: f32
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>
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
        .add_startup_system(setup_system)
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
        player_laser: asset_server.load(PLAYER_LASER_SPRITE)
    };
    commands.insert_resource(game_textures); // Add resource
}
