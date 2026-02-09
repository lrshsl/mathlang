mod graph;
mod message;
mod update;
mod view;

use graph::Graph;
use iced::widget::text_editor;

pub const ZOOM_DEFAULT: f64 = 2.0;
pub const ZOOM_WHEEL_SCALE: f64 = 0.05;

fn main() -> iced::Result {
    iced::application(MainState::new, MainState::update, MainState::view)
        .theme(iced::Theme::SolarizedDark)
        .run()
}

pub struct MainState {
    text: text_editor::Content,
    graph: Graph,
    err_msg: Option<String>,
}

impl MainState {
    pub fn new() -> Self {
        let mut s = Self {
            text: text_editor::Content::with_text("f(x) = sin(x);\nplot(f);"),
            graph: Graph::default(),
            err_msg: None,
        };
        s.on_text_change();
        s
    }
}
