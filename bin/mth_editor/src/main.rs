mod graph;
mod message;
mod update;
mod view;

use graph::Graph;

pub const ZOOM_DEFAULT: f64 = 2.0;
pub const ZOOM_WHEEL_SCALE: f64 = 0.2;

fn main() -> iced::Result {
    iced::application("MathLang", MainState::update, MainState::view)
        .theme(|_| iced::Theme::SolarizedDark)
        .run()
}

#[derive(Default)]
pub struct MainState {
    text: iced::widget::text_editor::Content,
    graph: Graph,
    err_msg: Option<String>,
}
