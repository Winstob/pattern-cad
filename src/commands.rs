use crate::core::CadTool;

#[derive(Debug)]
pub enum UiCommand {
    NewDocument,
    OpenDocument,
    SelectTool { tool: CadTool },
}
