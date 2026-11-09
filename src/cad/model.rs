use crate::cad::ids::ElementId;
use glam::*;
use std::collections::HashMap;

pub struct CadModel {
    pub points: HashMap<ElementId, CadModelPoint>,
}

impl CadModel {
    pub fn new() -> Self {
        CadModel {
            points: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }
}

pub struct CadModelPoint {
    pub location: Vec2,
}
