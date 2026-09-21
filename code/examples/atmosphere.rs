use bevy::prelude::*;
use bevy::light::Atmosphere;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::pbr::AtmosphereSettings;

fn setup(mut commands: Commands, mut media: ResMut<Assets<ScatteringMedium>>) {
    let medium = media.add(ScatteringMedium::default());

    commands.spawn((Atmosphere::earth(medium), Transform::from_xyz(0.0, -6_360_000.0, 0.0)));

    commands.spawn((
        Camera3d::default(),
        AtmosphereSettings::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}