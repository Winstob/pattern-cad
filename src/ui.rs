use crate::cad::tool::CadTool;
use crate::commands::UiCommand;

pub struct Ui {
    pending_commands: Vec<UiCommand>,
    pub cad_viewport_rect: Option<egui::Rect>,
    pub cad_viewport_input: ViewportInput,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            cad_viewport_rect: None,
            pending_commands: Vec::new(),
            cad_viewport_input: ViewportInput::default(),
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
        self.cad_viewport_rect = None;
        egui::CentralPanel::no_frame().show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            self.cad_viewport_rect = Some(rect);

            self.cad_viewport_input.focused = response.has_focus();
            self.cad_viewport_input.pointer_pos = response.hover_pos().map(|pos| {
                let local = pos - rect.min;
                Point2 {
                    x: local.x,
                    y: local.y,
                }
            });
        });
    }

    pub fn take_pending_commands(&mut self) -> Vec<UiCommand> {
        std::mem::take(&mut self.pending_commands)
    }
}

pub struct UiContext {
    pub model_available: bool,
}

#[derive(Default)]
pub struct ViewportInput {
    pub focused: bool,
    pub pointer_pos: Option<Point2>,
}

#[derive(Default, Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}
