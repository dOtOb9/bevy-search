use bevy::prelude::*;


#[derive(Component)]
struct Group;


fn setup(mut commands: Commands) {
    let group = commands
        .spawn((Group, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();

    
}