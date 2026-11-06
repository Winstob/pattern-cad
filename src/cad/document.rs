use super::expression::Expression;
use super::feature::Feature;
use super::geometry::point::*;
use super::model::CadModel;
use crate::cad::model::CadModelPoint;
use crate::common::types::Point2;

pub struct CadDocument {
    model: CadModel,

    history: Vec<Feature>,
    timeline_position: usize,

    next_point_id: usize,
}

impl CadDocument {
    pub fn new() -> Self {
        CadDocument {
            model: CadModel::new(),
            history: Vec::new(),
            timeline_position: 0,
            next_point_id: 0,
        }
    }

    fn allocate_point_id(&mut self) -> PointId {
        let point_id = PointId(self.next_point_id.into());
        self.next_point_id += 1;
        point_id
    }

    pub fn add_point(&mut self, def: PointDefinition) {
        println!("Add point: {:?}", def);
        let point_id = self.allocate_point_id();

        let feature = Feature::AddPoint {
            id: point_id,
            definition: def,
        };

        self.history.insert(self.timeline_position, feature);
        self.timeline_position += 1;

        self.regenerate();
    }

    fn regenerate(&mut self) {
        // regenerate model from feature history
        self.model.clear();
        for feature in self.history.iter() {
            match feature {
                Feature::AddPoint { id, definition } => {
                    let x: f32;
                    let y: f32;
                    match definition {
                        PointDefinition::Coordinates { x: raw_x, y: raw_y } => {
                            x = self.evaluate_expression(raw_x) as f32;
                            y = self.evaluate_expression(raw_y) as f32;
                        }
                    }
                    self.model.points.insert(
                        *id,
                        CadModelPoint {
                            location: Point2 { x, y },
                        },
                    );
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
