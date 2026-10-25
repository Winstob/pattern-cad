use crate::cad::tool::CadTool;
use crate::commands::UiCommand;
use crate::common::types::{Point2, Vec2};

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
                    if ui
                        .add_enabled(ui_context.model_available, egui::Button::new("Point"))
                        .clicked()
                    {
                        self.pending_commands.push(UiCommand::SelectTool {
                            tool: CadTool::DrawPoint,
                        });
                    }
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
                ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            self.cad_viewport_rect = Some(rect);

            self.cad_viewport_input.focused = response.has_focus();
            self.cad_viewport_input.pointer_pos = response.hover_pos().map(|pos| {
                let local = pos - rect.min;
                Point2 {
                    x: local.x,
                    y: local.y,
                }
            });
            self.cad_viewport_input.viewport_size = Vec2 {
                x: rect.width(),
                y: rect.height(),
            };

            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.cad_viewport_input
                        .pending_events
                        .push(ViewportEvent::Click {
                            position: Point2 {
                                x: (pos.x - rect.min.x),
                                y: (pos.y - rect.min.y),
                            },
                        });
                }
            }
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
    pub viewport_size: Vec2,
    pub pending_events: Vec<ViewportEvent>,
}

impl ViewportInput {
    pub fn take_pending_events(&mut self) -> Vec<ViewportEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

#[derive(Debug)]
pub enum ViewportEvent {
    Click { position: Point2 },
}
