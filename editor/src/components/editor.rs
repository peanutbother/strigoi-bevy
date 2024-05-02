use super::{tabviewer::EguiWindow, MenuBar, TabViewer};
use bevy::asset::UntypedAssetId;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use engine::prelude::*;
use std::any::TypeId;

#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, UntypedAssetId),
}

#[derive(Resource)]
pub struct Editor {
    state: DockState<EguiWindow>,
    viewport_rect: egui::Rect,
    pub(crate) selected_entities: SelectedEntities,
    selection: InspectorSelection,
}

impl Editor {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![EguiWindow::GameView]);
        let tree = state.main_surface_mut();
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![EguiWindow::Hierarchy]);
        let [_game, _bottom] = tree.split_below(
            game,
            0.8,
            vec![
                EguiWindow::Console,
                EguiWindow::Resources,
                EguiWindow::Assets,
            ],
        );

        Self {
            state,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    pub(crate) fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        MenuBar::top(world, ctx);

        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
        };

        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);

        MenuBar::bottom(world, ctx);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
