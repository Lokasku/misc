use bevy::prelude::*;


const PLAYER_SPRITE: &str = "ferry.png";
const PLAYER_SIZE: (f32, f32) = (100., 67.);


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
    .add_startup_system(setup)
    .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>) {

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    let window = windows.get_primary_mut().unwrap();
    let (win_width, win_height) = (window.width(), window.height());

    // Ferry
    let bottom = -win_width / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. + 5., 10.),
            ..Default::default()
        },
        ..Default::default()
    });
}