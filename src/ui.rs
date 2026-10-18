use crate::commands::UiCommand;

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

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_menu_bar(ui);
        self.show_central_panel(ui);
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
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
