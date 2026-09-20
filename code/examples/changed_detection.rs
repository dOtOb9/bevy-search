use bevy::prelude::*;


#[derive(Component)]
struct Position(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn(Position(0.0, 0.0));
}

fn on_added(query: Query<&Position, Added<Position>>) {
    for position in &query {
        println!("added: {} {}", position.0, position.1);
    }
}

fn move_right(mut query: Query<&mut Position>, time: Res<Time>) {
    for mut position in &mut query {
        position.0 += 10.0 * time.delta_secs();
    }
}

fn on_changed(query: Query<&Position, Changed<Position>>) {
    for position in &query {
        println!("changed: {} {}", position.0, position.1);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_right, on_added, on_changed).chain())
        .run();
}