use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Event)]
struct Bounced;

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(50.0, 0.0)));
}

fn movement(
    mut query: Query<(&mut Position, &mut Velocity),
    time: Res<Time>,
    mut bounced: EventWriter<Bounced>
) {
    
}