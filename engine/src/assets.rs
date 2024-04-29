use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetStage {
    #[default]
    Loading,
    Done,
}

plugin!(
    /// Asset loading plugin with loading states
    AssetPlugin, app => {
        app.init_state::<AssetStage>().add_loading_state(
            LoadingState::new(AssetStage::Loading).continue_to_state(AssetStage::Done),
        );
    }
);
