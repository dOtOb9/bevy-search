use bevy::prelude::*;

#[derive(Resource, Default)]
struct TickCount(u32);

fn setup(mut commands: Commands) {
    commands.insert_resource(TickCount::default());
}

fn fixed_tick(mut tick: ResMut<TickCount>, time: Res<Time>) {
    tick.0 += 1;
    println!("fixed tick {} (dt={:.5}s", tick.0, time.delta_secs());
}

fn update_tick(time: Res<Time>) {
    println!("update frame: (dt={:.5}s", time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fixed_tick)
        .add_systems(Update, update_tick)
        .run();
}