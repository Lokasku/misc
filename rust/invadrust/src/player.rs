use bevy::prelude::*;
use crate::{
    GameTextures,
    WinSize,
    PLAYER_SIZE,
    TIME_STEP,
    BASE_SPEED,
    PLAYER_LASER_SIZE,
    components::{
        Player,
        Velocity,
        Movable,
        FromPlayer,
        SpriteSize,
        Laser
    }
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
    ) {
    
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2.;

            // Player laser
            let mut spawn_laser = |x_offset: f32| {
                commands.spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + 19., 0.),
                        scale: Vec3::new(0.75, 0.75, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                .insert(Velocity {x: 0., y: 1.})
                .insert(Movable { auto_despawn: true });
            };

            spawn_laser(x_offset - 5.);
            spawn_laser(-x_offset + 5.);
        }
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut win_size: Res<WinSize>
    ) {

    // Ferry
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2., 10.), // X Y Z
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(SpriteSize::from(PLAYER_SIZE))
    .insert(Movable { auto_despawn: false })
    .insert(Velocity { x: 0., y: 0. });
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
    ) {

    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}
