use glam::DVec2;
use iced::{Rectangle, widget};

#[derive(Debug, Clone)]
pub enum Message {
    EditText(widget::text_editor::Action),
    PanningDelta(DVec2),
    ZoomDelta(DVec2, Rectangle, f64),
    SetError(String),
    ClearErrors,
}
