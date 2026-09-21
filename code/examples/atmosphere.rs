use bevy::prelude::*;
use bevy::light::Atmosphere;
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((Atmosphere::earth(), Transform::from_xyz(0.0, 0.0, 0.0)));

    commands.spawn((
        Camera3d::default(),
        AtmosphereSettings::defalut(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}