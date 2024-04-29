use engine::prelude::*;

/// State to determine focus of gameview
#[derive(Resource, Default)]
pub struct UiState {
    pub(crate) gameview_focus: bool,
}

/// User settings for interaction with the editor
#[derive(Resource)]
pub struct UiOptions {
    pub pancam: PanCamOptions,
    pub inspector: bool,
}

/// State to hold configuration of PanCam
pub struct PanCamOptions {
    pub enabled: bool,
    pub zoom_to_cursor: bool,
}

impl Default for UiOptions {
    fn default() -> Self {
        Self {
            pancam: default(),
            inspector: true,
        }
    }
}

impl Default for PanCamOptions {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_PANCAM_ENABLED,
            zoom_to_cursor: DEFAULT_PANCAM_ZOOM_TO_CURSOR,
        }
    }
}
