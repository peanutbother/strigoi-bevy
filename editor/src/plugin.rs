use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_mod_picking::{
    debug::DebugPickingMode,
    events::{Click, Pointer},
};
use bevy_pancam::PanCam;
use engine::prelude::*;

use crate::util::toggle_fullscreen;

plugin!(
    /// Plugin boilerplate to initialize egui, hotkeys and debugging plugins for the editor
    EditorPlugin, app => {
        app
        .add_plugins((
            bevy_inspector_egui::bevy_egui::EguiPlugin,
            bevy_inspector_egui::DefaultInspectorConfigPlugin,
            bevy_mod_picking::DefaultPickingPlugins
        ))
        .init_resource::<crate::state::UiState>()
        .init_resource::<crate::state::UiOptions>()
        .init_resource::<crate::components::Editor>()
        // setup editor
        .add_systems(
            PostUpdate,
            show_ui_system
                .before(bevy_inspector_egui::bevy_egui::EguiSet::ProcessOutput)
                .before(bevy::transform::TransformSystem::TransformPropagate),
        )
        .add_systems(Update, select_entity)
        // update camera
        .add_systems(Update, toggle_camera)
        // listen for hotkeys
        .add_systems(Update, listen_hotkeys);
    }
);

/// Scaffold egui ui
fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<crate::components::Editor, _>(|world, mut editor| {
        editor.ui(world, egui_context.get_mut())
    });
}

/// Selects an entity in inspector view when clicked
fn select_entity(
    state: Res<crate::state::UiState>,
    options: Res<crate::state::UiOptions>,
    mut egui_state: ResMut<crate::components::Editor>,
    mut event: EventReader<Pointer<Click>>,
) {
    if !state.gameview_focus || !options.inspector {
        event.clear();
        return;
    }

    for event in event.read() {
        egui_state.selected_entities.select_replace(event.target)
    }
}

/// Update PanCam settings on change
fn toggle_camera(
    options: Res<crate::state::UiOptions>,
    state: Res<crate::state::UiState>,
    mut camera: Query<&mut PanCam>,
) {
    if !options.is_changed() && !state.is_changed() {
        return;
    }

    let mut pan_cam = camera
        .get_single_mut()
        .expect("expected a single pancam instance");

    pan_cam.enabled = options.pancam.enabled && state.gameview_focus;
    pan_cam.zoom_to_cursor = options.pancam.zoom_to_cursor;
}

fn listen_hotkeys(
    key: Res<Input<KeyCode>>,
    debug: Res<State<DebugPickingMode>>,
    mut options: ResMut<crate::state::UiOptions>,
    mut next_debug: ResMut<NextState<DebugPickingMode>>,
    mut window: Query<&mut Window>,
) {
    let mod_key = key.pressed(KeyCode::ShiftLeft) || key.pressed(KeyCode::ShiftRight);

    for key in key.get_just_pressed() {
        match key {
            KeyCode::F1 => change_debug(&mut next_debug, &debug, mod_key),
            KeyCode::F2 => invert!(options.inspector),
            KeyCode::F3 => invert!(options.pancam.enabled),
            KeyCode::F4 => invert!(options.pancam.zoom_to_cursor),
            KeyCode::F12 => {
                if let Ok(mut window) = window.get_single_mut() {
                    toggle_fullscreen(&mut window);
                }
            }
            _ => {}
        }
    }
}

fn change_debug(
    next_debug: &mut ResMut<'_, NextState<DebugPickingMode>>,
    debug: &Res<'_, State<DebugPickingMode>>,
    mod_key: bool,
) {
    next_debug.set(match debug.get() {
        DebugPickingMode::Normal => {
            if mod_key {
                DebugPickingMode::Noisy
            } else {
                DebugPickingMode::Disabled
            }
        }
        DebugPickingMode::Noisy => {
            if mod_key {
                DebugPickingMode::Normal
            } else {
                DebugPickingMode::Disabled
            }
        }
        DebugPickingMode::Disabled => {
            if mod_key {
                DebugPickingMode::Noisy
            } else {
                DebugPickingMode::Normal
            }
        }
    })
}
