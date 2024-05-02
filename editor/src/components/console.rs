use bevy::ecs::system::SystemState;
use bevy_dev_console::{
    builtin_parser::Environment,
    command::{CommandHints, DefaultCommandParser, ExecuteCommand},
    config::ConsoleConfig,
    ui::{add_log, read_logs, ConsoleUiState, COMMAND_MESSAGE_NAME, COMMAND_MESSAGE_PREFIX},
};
use engine::prelude::*;

plugin!(
    /// Bootstrap Console widget and resources to provide an interactive console widget
    ConsolePlugin, app =>{
    app.init_resource::<ConsoleUiState>()
        .init_resource::<CommandHints>()
        .init_resource::<ConsoleConfig>()
        .register_type::<ConsoleConfig>()
        .init_non_send_resource::<Environment>()
        .init_resource::<DefaultCommandParser>()
        .add_systems(Update, read_logs);
});

pub type ConsoleState<'s> = SystemState<(
    Commands<'s, 's>,
    ResMut<'s, ConsoleUiState>,
    Res<'s, ButtonInput<KeyCode>>,
    ResMut<'s, CommandHints>,
    Res<'s, ConsoleConfig>,
)>;

pub struct Console<'w, 's: 'static> {
    world: &'w mut World,
    state: ConsoleState<'s>,
}

impl<'w> Console<'w, '_> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            state: ConsoleState::new(world),
            world,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let (mut commands, mut state, key, mut hints, config) = self.state.get_mut(self.world);
        let mut submit_command = |command: &mut String| {
            if !command.trim().is_empty() {
                info!(name: COMMAND_MESSAGE_NAME, "{COMMAND_MESSAGE_PREFIX}{}", command.trim());
                // Get the owned command string by replacing it with an empty string
                let command = std::mem::take(command);
                commands.add(ExecuteCommand(command));
            }
        };

        if key.just_pressed(config.submit_key) {
            submit_command(&mut state.command);
        }

        // A General rule when creating layouts in egui is to place elements which fill remaining space last.
        // Since immediate mode ui can't predict the final sizes of widgets until they've already been drawn

        // Thus we create a bottom panel first, where our text edit and submit button resides.
        egui::TopBottomPanel::bottom("bottom panel")
            .frame(egui::Frame::none().outer_margin(egui::Margin {
                left: 0.0,
                right: 5.0,
                top: 5. + 6.,
                bottom: 20.0,
            }))
            .show_inside(ui, |ui| {
                let text_edit_id = egui::Id::new("text_edit");

                // We can use a right to left layout, so we can place the text input last and tell it to fill all remaining space
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Submit").clicked() {
                        // submit_command(&mut state.command);

                        // Return keyboard focus to the text edit control.
                        ui.ctx().memory_mut(|mem| mem.request_focus(text_edit_id));
                    }
                    // ui.button is a shorthand command, a similar command exists for text edits, but this is how to manually construct a widget.
                    // doing this also allows access to more options of the widget, rather than being stuck with the default the shorthand picks.
                    let text_edit = egui::TextEdit::singleline(&mut state.command)
                        .id(text_edit_id)
                        .desired_width(ui.available_width())
                        .margin(egui::Vec2::splat(4.0))
                        // .font(config.theme.font.clone())
                        .lock_focus(true);

                    ui.add(text_edit);

                    // Each time we open the console, we want to set focus to the text edit control.
                    if !state.text_focus {
                        state.text_focus = true;
                        ui.ctx().memory_mut(|mem| mem.request_focus(text_edit_id));
                    }
                });
            });
        // Now we can fill the remaining minutespace with a scrollarea, which has only the vertical scrollbar enabled and expands to be as big as possible.
        egui::ScrollArea::new([false, true])
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    let mut command_index = 0;

                    for (id, (message, is_new)) in state.log.iter_mut().enumerate() {
                        add_log(
                            ui,
                            id,
                            message,
                            is_new,
                            &mut hints,
                            &config,
                            &mut command_index,
                        );
                    }
                });
            });
        // });
        self.state.apply(self.world);
    }
}
