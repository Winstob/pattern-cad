use crate::common::types::{Point2, Vec2};

pub struct CadView {
    pub zoom: f32,
    pub center: Point2,
}

impl CadView {
    pub fn default() -> Self {
        CadView {
            zoom: 1.0,
            center: Point2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn viewport_to_world(&self, pos: &Point2, viewport_size: &Vec2) -> Point2 {
        Point2 {
            x: self.center.x + (pos.x - viewport_size.x / 2.0) / self.zoom,
            y: self.center.y + (pos.y - viewport_size.y / 2.0) / self.zoom,
        }
    }
}
