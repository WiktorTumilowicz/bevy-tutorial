pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use events::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (handle_game_over, exit_game))
        .run();
}
