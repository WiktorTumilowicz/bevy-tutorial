use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use crate::AppState;

use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .init_state::<SimulationState>()
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
