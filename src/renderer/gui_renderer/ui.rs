pub struct Ui {
    pub cad_draw_viewport: Option<egui::Rect>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            cad_draw_viewport: None,
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
                    ui.button("New");
                });
            });
        });
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {
        // must be called last
        self.cad_draw_viewport = None;
        egui::CentralPanel::no_frame().show(ui, |ui| {
            let (rect, _response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::hover(),
            );
            self.cad_draw_viewport = Some(rect);
        });
    }
}
