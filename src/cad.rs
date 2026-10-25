pub mod model;
pub mod tool;
pub mod view;

use crate::commands::UiCommand;
use crate::ui::{ViewportEvent, ViewportInput};
use model::Model;
use tool::CadTool;
use view::CadView;

pub struct CadEngine {
    model: Option<Model>,
    active_tool: CadTool,
    view: CadView,
}

impl CadEngine {
    pub fn new() -> Self {
        CadEngine {
            model: None,
            active_tool: CadTool::Select,
            view: CadView::default(),
        }
    }

    pub fn handle_ui_command(&mut self, cmd: UiCommand) {
        match cmd {
            UiCommand::NewDocument => {
                self.model = Some(Model::new());
                self.active_tool = CadTool::Select;
            }
            UiCommand::OpenDocument => {
                println!("Open File!");
                self.active_tool = CadTool::Select;
            }
            UiCommand::SelectTool { tool } => {
                self.active_tool = tool;
            }
        }
    }

    pub fn handle_input(&mut self, input: &mut ViewportInput) {
        for event in input.take_pending_events() {
            match (&self.active_tool, event) {
                (CadTool::DrawPoint, ViewportEvent::Click { position }) => {
                    let world_pos = self.view.viewport_to_world(&position, &input.viewport_size);
                    self.model
                        .as_mut()
                        .expect("Model should be loaded before adding geometry!")
                        .add_point(world_pos);
                }
                (tool, event) => todo!(
                    "input event not yet handled! (tool: [{:?}] event: [{:?}]",
                    tool,
                    event
                ),
            }
        }
    }

    pub fn model_loaded(&self) -> bool {
        self.model.is_some()
    }
}
