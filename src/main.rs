use engine::prelude::*;

fn main() {
    engine::new(Some(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            title: WINDOW_TITLE.to_owned(),
            ..default()
        }),
        ..default()
    }))
    // setup game
    .add_systems(Startup, setup)
    // start level
    .insert_resource(LevelSelection::default())
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    engine::camera::create_camera(&mut commands, DEFAULT_PANCAM_ENABLED);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(DEFAULT_MAP_PATH),
        ..default()
    });
}
