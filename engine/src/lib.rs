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

/// Boilerplate to create a bevy plugin
#[macro_export]
macro_rules! plugin {
    ($(#[$($attr:tt)*])* $plugin_name:ident, $app:ident => {
        $($fn_body:tt)*
    }) => {
        $(#[$($attr)*])*
        pub struct $plugin_name;
        impl Plugin for $plugin_name {
            fn build(&self, $app: &mut App) {
                $($fn_body)*
            }
        }
    };
}

/// Registers an LDTK entity using the blueprint pattern (post processing the entity in a designated system) with its `bindings::IDENTIFIER`
#[macro_export]
macro_rules! register_entity {
    ($app:ident, $bundle:ty) => {
        $app.register_ldtk_entity::<$bundle>(bindings::IDENTIFIER)
            .add_systems(Update, bindings::process_entity)
    };
}

/// Registers an ldtk int cell bundle using the module's `bindings::INT_GRID_VALUE`
#[macro_export]
macro_rules! register_int_cell {
    ($app:ident, $bundle:ty) => {
        $app.register_ldtk_int_cell::<$bundle>(bindings::INT_GRID_VALUE)
    };
}

/// Register one or more types with bevy
#[macro_export]
macro_rules! register_types {
    ($app:ident, $($register_type:ty),*) => {
        $app
            $(.register_type::<$register_type>())*
    };
}

/// This macro inverts a mutable boolean variable in place
#[macro_export]
macro_rules! invert {
    ($var:ident) => {
        $var = !$var
    };
    ($var:expr) => {
        $var = !$var
    };
}
