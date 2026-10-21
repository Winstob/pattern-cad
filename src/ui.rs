use crate::commands::UiCommand;
use crate::core::CadTool;

pub struct Ui {
    pub cad_draw_viewport: Option<egui::Rect>,
    pending_commands: Vec<UiCommand>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            cad_draw_viewport: None,
            pending_commands: Vec::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, ui_context: &UiContext) {
        self.show_menu_bar(ui, ui_context);
        self.show_central_panel(ui);
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui, ui_context: &UiContext) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.pending_commands.push(UiCommand::NewDocument);
                    }
                    if ui.button("Open File").clicked() {
                        self.pending_commands.push(UiCommand::OpenDocument);
                    }
                });
                ui.menu_button("Tools", |ui| {
                    ui.menu_button("Line", |ui| {
                        if ui
                            .add_enabled(ui_context.model_available, egui::Button::new("Line"))
                            .clicked()
                        {
                            self.pending_commands.push(UiCommand::SelectTool {
                                tool: CadTool::DrawLine,
                            });
                        }
                    });
                });
            });
        });
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {
        // must be called last
        self.cad_draw_viewport = None;
        egui::CentralPanel::no_frame().show(ui, |ui| {
            let (rect, _response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            self.cad_draw_viewport = Some(rect);
        });
    }

    pub fn take_pending_commands(&mut self) -> Vec<UiCommand> {
        std::mem::take(&mut self.pending_commands)
    }
}

pub struct UiContext {
    pub model_available: bool,
}
