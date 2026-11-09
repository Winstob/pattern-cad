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

    pub fn viewport_to_world(&self, pos: &Vec2, viewport_size: &Vec2, pixels_per_mm: f32) -> Vec2 {
        let scale = pixels_per_mm * self.zoom;
        Vec2 {
            x: self.center.x + (pos.x - viewport_size.x / 2.0) / scale,
            y: self.center.y - (pos.y - viewport_size.y / 2.0) / scale,
        }
    }

    pub fn world_to_ndc_matrix(&self, viewport_size: &Vec2, pixels_per_mm: f32) -> Mat4 {
        let scale_x = 2.0 * self.zoom * pixels_per_mm / viewport_size.x;
        let scale_y = -(2.0 * self.zoom * pixels_per_mm / viewport_size.y);

        Mat4::from_cols(
            Vec4::new(scale_x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale_y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(-self.center.x * scale_x, -self.center.y * scale_y, 0.0, 1.0),
        )
    }
}
