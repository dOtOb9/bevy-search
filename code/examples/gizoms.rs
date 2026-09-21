use bevy::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Position(-100.0, 0.0));
    commands.spawn(Position(100.0, 0.0));
    commands.spawn(Position(0.0, 80.0));
}

fn draw_agents(mut gizmos: Gizmos, query: Query<&Position>) {
    for position in &query {
        gizmos.circle_2d(
            Vec2::new(position.0, position.1),
            20.0,
            Color::srgb(0.9, 0.2, 0.2),
        );
    }

    gizmos.line_2d(
        Vec2::new(-200.0, -150.0),
        Vec2::new(200.0, -150.0),
        Color::srgb(0.5, 0.5, 0.5),
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_agents)
        .run();
}