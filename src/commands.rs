use crate::cad::tool::CadTool;

#[derive(Debug)]
pub enum UiCommand {
    NewDocument,
    OpenDocument,
    SelectTool { tool: CadTool },
}
