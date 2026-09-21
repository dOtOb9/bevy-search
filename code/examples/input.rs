use bevy::prelude::*;

#[derive(Resource, Default)]
struct Running(bool);


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Running(true));
}

fn toggle_pause(keys: Res<ButtonInput<KeyCode>>, mut running: ResMut<Running>) {
    if keys.just_pressed(KeyCode::Space) {
        running.0 = !running.0;

        println!("running: {}", running.0);
    }
}

fn click_position(mouse: Res<ButtonInput<MouseButton>>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(position) = window.cursor_position() {
                println!("clicked at {:?}", position);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_pause, click_position))
        .run();
}