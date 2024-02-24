use bevy::prelude::*;

use resources::*;
use systems::*;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(
                Update,
                (update_score, update_high_scores, high_scores_updated),
            );
    }
}
