use bevy::prelude::*;


#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(1.0, 0.5)));
    commands.spawn((Position(10.0, 10.0), Velocity(-1.0, 0.5)));
}

fn movement(mut query: Query<(&mut Position, &Velocity)) {
    for (mut position, velocity) in &mut query {
        
    }
}