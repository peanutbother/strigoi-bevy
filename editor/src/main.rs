use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use engine::prelude::*;

mod components;
mod plugin;
mod state;
mod util;

fn main() {
    engine::new(Some(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            title: WINDOW_TITLE.to_owned() + " | EDITOR",
            ..default()
        }),
        ..default()
    }))
    // diagnostics
    .add_plugins((
        // LogDiagnosticsPlugin,
        FrameTimeDiagnosticsPlugin,
    ))
    // editor
    .add_plugins(plugin::EditorPlugin)
    // setup game
    .add_systems(Startup, setup)
    // start level
    .insert_resource(LevelSelection::default())
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    engine::camera::create_camera(&mut commands, true);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(DEFAULT_MAP_PATH),
        ..default()
    });
}
