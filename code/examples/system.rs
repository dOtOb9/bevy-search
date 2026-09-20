use bevy::prelude::*;

#[derive(Resource, Default)]
struct Counter(u32);

fn setup(mut commands: Commands) {
    commands.insert_resource(Counter::default());
}

fn count(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

fn print_count(counter: ResMut<Counter>) {
    println!("count: {}", counter.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (count, print_count).chain())
        .run();
}