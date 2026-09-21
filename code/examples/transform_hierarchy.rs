use bevy::prelude::*;


#[derive(Component)]
struct Group;


fn setup(mut commands: Commands) {
    let group = commands
        .spawn((Group, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();

    let member_a = commands.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let member_b = commands.spawn(Transform::from_xyz(-10.0, 0.0, 0.0)).id();

    commands.entity(group).add_children(&[member_a, member_b]);
}

fn move_group(mut query: Query<&mut Transform, With<Group>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x += 20.0 * time.delta_secs();
    }
}

fn print_members(query: Query<&GlobalTransform, Without<Group>>) {
    for global_transform in &query {
        println!("member at {:?}", global_transform.translation());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_group, print_members).chain())
        .run();
}