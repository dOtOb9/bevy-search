use bevy::prelude::*;
use std::time::Instant;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    let agents = 
        (0..50_000)
        .map(|i| (Position(i as f32, 0.0), Velocity(1.0, 0.5)));

    commands.spawn_batch(agents);
}

fn movement(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    let start = Instant::now();

    query.par_iter_mut().for_each(|(mut position, velocity)| {
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;
    });

    println!("movement took {:?}", start.elapsed());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}