use super::element::point::*;
use super::expression::Expression;
use super::feature::Feature;
use super::model::CadModel;
use crate::cad::element::{Element, ElementDefinition};
use crate::cad::ids::*;
use crate::cad::model::CadModelPoint;
use glam::*;

pub struct CadDocument {
    model: CadModel,

    history: Vec<Feature>,
    timeline_position: usize,

    next_element_id: ElementId,
    next_feature_id: FeatureId,
}

impl CadDocument {
    pub fn new() -> Self {
        CadDocument {
            model: CadModel::new(),
            history: Vec::new(),
            timeline_position: 0,
            next_element_id: ElementId(0),
            next_feature_id: FeatureId(0),
        }
    }

    pub fn get_model(&self) -> &CadModel {
        &self.model
    }

    fn allocate_element_id(&mut self) -> ElementId {
        let id = self.next_element_id;
        self.next_element_id = ElementId(self.next_element_id.0 + 1);
        id
    }

    fn allocate_feature_id(&mut self) -> FeatureId {
        let id = self.next_feature_id;
        self.next_feature_id = FeatureId(self.next_feature_id.0 + 1);
        id
    }

    pub fn add_point(&mut self, def: PointDefinition) {
        println!("Add point: {:?}", def);

        let element_id = self.allocate_element_id();
        let point = Element {
            id: element_id,
            def: ElementDefinition::Point(def),
        };

        let feature_id = self.allocate_feature_id();
        let feature = Feature {
            id: feature_id,
            elements: vec![point],
        };

        self.history.insert(self.timeline_position, feature);
        self.timeline_position += 1;

        self.regenerate();
    }

    fn regenerate(&mut self) {
        // regenerate model with resolved coordinates from feature history
        self.model.clear();
        for feature in self.history.iter() {
            for element in feature.elements.iter() {
                match &element.def {
                    ElementDefinition::Point(definition) => {
                        let x: f32;
                        let y: f32;
                        match definition {
                            PointDefinition::Coordinates { x: raw_x, y: raw_y } => {
                                x = self.evaluate_expression(&raw_x) as f32;
                                y = self.evaluate_expression(&raw_y) as f32;
                            }
                        }
                        self.model.points.insert(
                            element.id,
                            CadModelPoint {
                                location: Vec2 { x, y },
                            },
                        );
                    }
                }
            }
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> f64 {
        match expression {
            Expression::Constant(val) => *val,
        }
    }
}
