use crate::prelude::*;

mod player;

pub mod bindings {
    pub use super::player::bindings as player;
}

plugin!(
    /// Bundle all assets and entities from LDTK with their bindings
    BundlePlugin, app => {
    app
        .register_type::<Worldly>()
        .add_plugins(
            player::PlayerPlugin
        );
});
