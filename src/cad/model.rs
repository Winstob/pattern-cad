use super::tool::CadTool;

pub struct Model {
    tool_selected: CadTool,
}

impl Model {
    pub fn new() -> Self {
        Model {
            tool_selected: CadTool::Select,
        }
    }

    pub fn select_tool(&mut self, tool: CadTool) {
        self.tool_selected = tool;
    }
}
