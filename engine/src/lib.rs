//! Base crate with functionality and boilerplate to create a game and editor

use bevy_ecs_ldtk::{LdtkSettings, LevelSpawnBehavior};

pub mod assets;
pub mod bundles;
pub mod camera;
pub mod config;
pub mod prelude;

pub mod bindings {
    pub use super::bundles::bindings::*;
}

/// Create the base engine with all default plugins, bindings, assets, configuration and systems registered
pub fn new(init: Option<bevy::window::WindowPlugin>) -> bevy::prelude::App {
    let mut app = bevy::app::App::new();
    app.add_plugins(
        bevy::app::PluginGroup::set(
            bevy::prelude::DefaultPlugins,
            bevy::prelude::ImagePlugin::default_nearest(),
        )
        // configure window
        .set(init.unwrap_or_default()),
    )
    // disable MSAA
    .insert_resource(bevy::render::view::Msaa::Off)
    // load bevy_ecs_ldtk
    .add_plugins(bevy_ecs_ldtk::LdtkPlugin)
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
        ..Default::default()
    });

    app
}
