use bevy::prelude::*;


#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Resource, Default)]
struct TotalDistance(f32);

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(1.0, 0.0)));
    commands.spawn((Position(5.0, 5.0), Velocity(0.0, 2.0)));
    commands.insert_resource(TotalDistance::default());
}

fn movement(mut query: Query<(&mut Position, &Velocity)>, mut total: ResMut<TotalDistance>) {
    for (mut position, velocity) in query {
        position.0 += velocity.0;
        position.1 += velocity.1;

        total.0 += ((velocity.0).powi(2) + (velocity.1).powi(2)).sqrt();
    }
}

fn print_total(total: Res<TotalDistance>) {
    println!("total distance: {}", total.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, print_total)
        .run();
}