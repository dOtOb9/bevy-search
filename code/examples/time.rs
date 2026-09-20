use bevy::prelude::*;


#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(50.0, 0.0)));
}

fn movement(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;
    }
}

fn print_position(query: Query<&Position>) {
    
}