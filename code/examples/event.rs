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
    mut query: Query<(&mut Position, &mut Velocity)>,
    time: Res<Time>,
    mut bounced: EventWriter<Bounced>
) {
    let dt = time.delta_secs();
    for (position, velocity) in &mut query {
        position.0 += velocity.0 * dt;

        if position.0 > 100.0 || position.0 < 0.0 {
            velocity.0 = -velocity.0;
            bounced.send(Bounced);
        }
    }
}

fn on_bounced(mut events: EventReader<Bounced>) {
    for _ in events.read() {
        println!("bounced!");    
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<Bounced>()
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, on_bounced).chain())
        .run();
}