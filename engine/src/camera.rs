use crate::prelude::*;
use bevy_pancam::PanCam;

#[derive(Component)]
pub struct MainCamera;

/// Create a basic Camera2D with PanCam plugin and configuration
pub fn create_camera(commands: &mut Commands, pan: bool) {
    let camera = Camera2dBundle::default();

    let pan_cam = PanCam {
        grab_buttons: vec![MouseButton::Middle],
        enabled: pan,
        zoom_to_cursor: DEFAULT_PANCAM_ZOOM_TO_CURSOR,
        ..default()
    };

    commands.spawn((MainCamera, camera, pan_cam));
}
