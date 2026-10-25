pub mod model;
pub mod tool;

use crate::commands::UiCommand;
use model::Model;

pub struct CadEngine {
    model: Option<Model>,
}

impl CadEngine {
    pub fn new() -> Self {
        CadEngine { model: None }
    }

    pub fn handle_ui_command(&mut self, cmd: UiCommand) {
        match cmd {
            UiCommand::NewDocument => {
                self.model = Some(Model::new());
            }
            UiCommand::OpenDocument => println!("Open File!"),
            UiCommand::SelectTool { tool } => {
                self.model
                    .as_mut()
                    .expect("Model should be loaded before being allowed to select a tool")
                    .select_tool(tool);
            } //_ => eprintln!("Unhandled UiCommand [{:?}]", cmd),
        }
    }

    pub fn model_loaded(&self) -> bool {
        self.model.is_some()
    }
}
