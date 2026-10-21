pub struct CadModel {
    tool_selected: CadTool,
}

impl CadModel {
    pub fn new() -> Self {
        CadModel {
            tool_selected: CadTool::Select,
        }
    }

    pub fn select_tool(&mut self, tool: CadTool) {
        self.tool_selected = tool;
    }
}

#[derive(Debug)]
pub enum CadTool {
    Select,
    DrawLine,
}
