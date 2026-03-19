use crate::PLOT_TYPE_NO_PLOT;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct PlotDesc {
    pub length: u32,
    pub type_id: u32,
    pub _pad: u64, // 16-byte alignment
}

impl Default for PlotDesc {
    fn default() -> Self {
        Self {
            length: 0,
            type_id: PLOT_TYPE_NO_PLOT,
            _pad: 0,
        }
    }
}
