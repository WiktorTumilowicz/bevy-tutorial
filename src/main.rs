pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScore>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_stars, spawn_player, spawn_enemies),
        )
        .add_systems(
            Update,
            (
                player_movement,
                update_enemy_directon,
                enemy_movement,
                confine_player_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
                tick_star_spawn_timer,
                spawn_stars_over_time,
                handle_game_over,
                update_high_scores,
                high_scores_updated,
            ),
        )
        .run();
}
