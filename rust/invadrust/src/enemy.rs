use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::{
    GameTextures,
    WinSize,
    ENEMY_SIZE,
    components::{
        Enemy,
        SpriteSize
    }
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
    ) {

    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    // Invaders
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.enemy.clone(), 
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Enemy)
    .insert(SpriteSize::from(ENEMY_SIZE));
}
