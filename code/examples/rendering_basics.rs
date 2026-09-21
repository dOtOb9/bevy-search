use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let handle: Handle<Image> = asset_server.load("icon.png");

    commands.spawn((
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(-100.0, 0.0, 0.0)
    ));

    commands.spawn((
        Sprite::from_image(handle),
        Transform::from_xyz(100.0, 0.0, 0.0)
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}