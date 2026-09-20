use bevy::prelude::*;


#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Player;


fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Player));
    commands.spawn(Position(5.0, 5.0));
}

fn list_entities(query: Query<(Entity, &Position)>) {
    for (entity, position) in &query {
        println!("{entity:?}: ({}, {})", position.0, position.1);
    }
}

fn despawn_non_players(
    mut commands: Commands,
    query: Query<Entity, (With<Position>, Without<Player>)>
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (list_entities, despawn_non_players).chain())
        .run();
}