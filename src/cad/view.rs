use glam::*;

pub struct CadView {
    pub zoom: f32,
    pub center: Vec2,
}

impl CadView {
    pub fn default() -> Self {
        CadView {
            zoom: 1.0,
            center: Vec2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn viewport_to_world(&self, pos: &Vec2, viewport_size: &Vec2) -> Vec2 {
        Vec2 {
            x: self.center.x + (pos.x - viewport_size.x / 2.0) / self.zoom,
            y: self.center.y + (pos.y - viewport_size.y / 2.0) / self.zoom,
        }
    }
}
