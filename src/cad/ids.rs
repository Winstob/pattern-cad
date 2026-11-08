#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct ElementId(pub u32); // need to be compatible with gpu types, so this should
// probably stay u32.

#[derive(Debug, Copy, Clone)]
pub struct FeatureId(pub u32);
