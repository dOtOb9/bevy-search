use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum SimPhase {
    #[default]
    Running, 
    Paused,
}

#[derive(Resource, Default)]
struct StepCount(u32);

fn setup(mut commands: Commands) {
    commands.insert_resource(StepCount::default());
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimPhase>>,
    mut next_state: ResMut<NextState<SimPhase>>
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            SimPhase::Running   => next_state.set(SimPhase::Paused),
            SimPhase::Paused    => next_state.set(SimPhase::Running),
        }
    }
}

fn simulate(mut step: ResMut<StepCount>) {
    step.0 += 1;
    println!("step: {}", step.0);
}

fn on_enter_paused() {
    println!("--- paused ---");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<SimPhase>()
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_pause)
        .add_systems(FixedUpdate, simulate.run_if(in_state(SimPhase::Running)))
        .add_systems(OnEnter(SimPhase::Paused), on_enter_paused)
        .run();
}