pub mod point;

use crate::cad::ids::ElementId;
use point::*;

#[derive(Debug)]
pub struct Element {
    pub id: ElementId,
    pub def: ElementDefinition,
}

#[derive(Debug)]
pub enum ElementDefinition {
    Point(PointDefinition),
}
