use super::geometry::point::*;

#[derive(Debug)]
pub enum Feature {
    AddPoint {
        id: PointId,
        definition: PointDefinition,
    },
}
