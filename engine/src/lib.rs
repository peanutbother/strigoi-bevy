//! Base crate with functionality and boilerplate to create a game and editor
use bevy::{log::LogPlugin, prelude::*};
#[cfg(feature = "editor")]
use bevy_dev_console::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod assets;
pub mod bundles;
pub mod camera;
pub mod config;
pub mod prelude;

pub mod bindings {
    pub use super::bundles::bindings::*;
}

/// Create the base engine with all default plugins, bindings, assets, configuration and systems registered
pub fn new(init: Option<WindowPlugin>) -> App {
    let mut app = App::new();
    app.add_plugins((
        #[cfg(feature = "editor")]
        ConsoleLogPlugin::default(),
        if cfg!(feature = "editor") {
            PluginGroup::set(DefaultPlugins, ImagePlugin::default_nearest()).disable::<LogPlugin>()
        } else {
            PluginGroup::set(DefaultPlugins, ImagePlugin::default_nearest())
        }
        // configure window
        .set(init.unwrap_or_default()),
    ))
    // disable MSAA
    .insert_resource(Msaa::Off)
    // load bevy_ecs_ldtk
    .add_plugins(LdtkPlugin)
    // load assets
    .add_plugins(assets::AssetPlugin)
    // load game bundles (bindings and systems)
    .add_plugins(bundles::BundlePlugin)
    // load camera plugin
    .add_plugins(bevy_pancam::PanCamPlugin)
    // configure bevy_ecs_ldtk
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        ..default()
    });

    app
}
