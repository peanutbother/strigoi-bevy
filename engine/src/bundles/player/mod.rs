use crate::prelude::*;

mod assets;
pub mod bindings;
pub mod components;
mod systems;

plugin!(
    /// Bind Player entity with its assets and systems
    PlayerPlugin, app => {
        register_entity!(app, bindings::PlayerBundle)
            .configure_loading_state(
                LoadingStateConfig::new(AssetStage::Loading)
                    .load_collection::<assets::PlayerAssets>(),
            )
            .insert_resource(components::MovementTimer(Timer::from_seconds(
                0.08,
                TimerMode::Repeating,
            )))
            .add_systems(Update, (
                systems::select_level_from_pos,
                systems::move_player_from_input,
                systems::translate_grid_coords_entities,
            ));
    }
);
