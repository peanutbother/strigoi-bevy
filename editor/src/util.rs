use bevy::window::WindowMode;
use engine::prelude::*;

/// toggles between fullscreen and windowed mode
pub fn toggle_fullscreen(window: &mut Window) {
    window.mode = match window.mode {
        WindowMode::Windowed => WindowMode::Fullscreen,
        _ => WindowMode::Windowed,
    };
}
