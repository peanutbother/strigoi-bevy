use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    window::WindowMode,
};
use bevy_mod_picking::debug::DebugPickingMode;
use egui::{menu, Context, TopBottomPanel};
use engine::prelude::*;

use crate::{state::UiOptions, util::toggle_fullscreen};
pub struct MenuBar;

impl MenuBar {
    pub(crate) fn top(world: &mut World, ctx: &mut Context) {
        TopBottomPanel::top("menubar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Editor", |ui| {
                    if let Ok(mut window) = world.query::<&mut Window>().get_single_mut(world) {
                        if ui
                            .checkbox(
                                &mut matches!(window.mode, WindowMode::Fullscreen),
                                "Fullscreen",
                            )
                            .clicked()
                        {
                            toggle_fullscreen(&mut window);
                        }
                    }
                });

                ui.menu_button("Settings", |ui| {
                    ui.menu_button("Debug (F1)", |ui| {
                        let mut debug_state = *world.resource_mut::<DebugPickingMode>();

                        let debug_enabled = !matches!(debug_state, DebugPickingMode::Disabled);

                        ui.checkbox(
                            &mut world.resource_mut::<UiOptions>().inspector,
                            "Click to inspect entities (F2)",
                        );

                        if ui
                            .checkbox(&mut debug_enabled.clone(), "Debug info on hover (F1)")
                            .clicked()
                        {
                            debug_state = match debug_state {
                                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                                    _ => DebugPickingMode::Disabled,
                            };
                        }

                        ui.set_enabled(debug_enabled);
                        if ui
                            .checkbox(
                                &mut (debug_state == DebugPickingMode::Noisy),
                                "Enable verbose Debug on hover (Shift+F1)",
                            )
                            .clicked()
                        {
                            debug_state = match debug_state {
                                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
                                    DebugPickingMode::Noisy => DebugPickingMode::Normal,
                                    _ => DebugPickingMode::Disabled,
                            };
                        }

                        *world.resource_mut::<DebugPickingMode>() = debug_state;
                    });

                    ui.menu_button("PanCam (F3)", |ui| {
                        ui.checkbox(
                            &mut world.resource_mut::<UiOptions>().pancam.enabled,
                            "Enable (F3)",
                        );
                        // ui.set_enabled(world.resource_mut::<UiOptions>().pancam.enabled);
                        ui.checkbox(
                            &mut world.resource_mut::<UiOptions>().pancam.zoom_to_cursor,
                            "Zoom to Cursor (F4)",
                        );
                    });
                });

                ui.set_enabled(false);
                ui.menu_button("Help", |_ui| {
                    // TODO
                });
            });
        });
    }

    pub(crate) fn bottom(world: &mut World, ctx: &mut Context) {
        TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                if let Some(diags) = world.get_resource::<DiagnosticsStore>() {
                    let fps = diags
                        .get(&FrameTimeDiagnosticsPlugin::FPS)
                        .map(|d| d.average().unwrap_or(0.0))
                        .unwrap_or(0.0);
                    let ft = diags
                        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                        .map(|d| d.average().unwrap_or(0.0))
                        .unwrap_or(0.0);
                    ui.label(format!("fps: {:.0} | frame time: {:.0}ms", fps, ft));
                }
            });
        });
    }
}
