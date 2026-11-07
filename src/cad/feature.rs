use crate::cad::element::Element;
use crate::cad::ids::FeatureId;

#[derive(Debug)]
pub struct Feature {
    pub id: FeatureId,
    pub elements: Vec<Element>,
}
