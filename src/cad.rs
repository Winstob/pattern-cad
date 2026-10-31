mod document;
mod expression;
mod feature;
mod geometry;
mod model;
pub mod tool;
pub mod view;

use crate::commands::UiCommand;
use crate::ui::{ViewportEvent, ViewportInput};
use document::CadDocument;
use expression::Expression;
use geometry::point::PointDefinition;
use tool::CadTool;
use view::CadView;

pub struct CadEngine {
    document: Option<CadDocument>,
    active_tool: CadTool,
    view: CadView,
}

impl CadEngine {
    pub fn new() -> Self {
        CadEngine {
            document: None,
            active_tool: CadTool::Select,
            view: CadView::default(),
        }
    }

    pub fn handle_ui_command(&mut self, cmd: UiCommand) {
        match cmd {
            UiCommand::NewDocument => {
                self.document = Some(CadDocument::new());
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
                    self.document
                        .as_mut()
                        .expect("Model should be loaded before adding geometry!")
                        .add_point(PointDefinition::Coordinates {
                            x: Expression::Constant(world_pos.x.into()),
                            y: Expression::Constant(world_pos.y.into()),
                        });
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
        self.document.is_some()
    }
}
