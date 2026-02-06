use glam::DVec2;

use crate::graph_shader_pipeline::ZOOM_PIXELS_FACTOR;

#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub zoom: f64,
    pub offset: DVec2,
}

impl Controls {
    pub fn pixel_ratio(&self) -> f64 {
        1.0 / 2.0_f64.powf(self.zoom) / ZOOM_PIXELS_FACTOR
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            zoom: 1.,
            offset: DVec2::ZERO,
        }
    }
}
