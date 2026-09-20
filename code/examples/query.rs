use bevy::prelude::*;


#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(1.0, 0.5)));
    commands.spawn((Position(10.0, 10.0), Velocity(-1.0, 0.5)));
}

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0;
        position.1 += velocity.1;
    }
}

fn print_positions(query: Query<&Position>) {
    for position in &query {
        println!("({}, {})", position.0, position.1);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, print_positions))
        .run();
}