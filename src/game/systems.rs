use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
                println!("Simulation Paused.");
            }
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
                println!("Simulation Runnning.");
            }
        };
    }
}
