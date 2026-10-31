use super::super::expression::Expression;

#[derive(Debug)]
pub struct PointId(pub usize);

#[derive(Debug)]
pub enum PointDefinition {
    Coordinates { x: Expression, y: Expression },
}
