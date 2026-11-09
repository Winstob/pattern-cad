#[derive(Debug)]
pub struct DisplayMetrics {
    pub pixels_per_mm: f32,
}

impl Default for DisplayMetrics {
    fn default() -> Self {
        Self {
            pixels_per_mm: 96.0 / 25.4, // 96 DPI
        }
    }
}
