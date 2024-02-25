pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((MainMenuPlugin, GamePlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                transition_to_game_state,
                transition_to_main_menu_state,
                handle_game_over,
                exit_game,
            ),
        )
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
