use crate::common::types::Point2;

pub struct Model {}

impl Model {
    pub fn new() -> Self {
        Model {}
    }

    pub fn add_point(&mut self, pos: Point2) {
        println!("Add point: {:?}", pos);
    }
}
