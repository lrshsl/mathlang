use std::sync::Arc;

use glam::DVec2;
use iced::{
    Element,
    Length::{Fill, FillPortion},
    Rectangle, Theme,
    widget::{self, column, container, row, text, vertical_space},
};

mod parser;

mod graph;
use graph::Graph;

use crate::parser::{cursor::Cursor, parse_program};

pub const ZOOM_DEFAULT: f64 = 2.0;
pub const ZOOM_WHEEL_SCALE: f64 = 0.2;

fn main() -> iced::Result {
    iced::application("MathLang", MainState::update, MainState::view)
        .theme(|_| Theme::SolarizedDark)
        .run()
}

#[derive(Default)]
pub struct MainState {
    text: widget::text_editor::Content,
    graph: Graph,
    err_msg: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditText(widget::text_editor::Action),
    PanningDelta(DVec2),
    UpdateZoom(f64),
    ZoomDelta(DVec2, Rectangle, f64),
    SetError(String),
    ClearErrors,
}

impl MainState {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::EditText(action) => {
                self.text.perform(action);
                let text = self.text.text();
                match parse_program(&text) {
                    Ok((_rem, prog)) => {
                        self.graph.instructions = Arc::new(prog);
                        self.graph.instructions_dirty = true;
                        self.update(Message::ClearErrors);
                    }
                    Err(e) => self.update(Message::SetError(format!("{e}"))),
                }
            }
            Message::PanningDelta(delta) => {
                self.graph.controls.center -= 2.0 * delta * self.graph.controls.scale();
            }
            Message::UpdateZoom(zoom) => {
                self.graph.controls.zoom = zoom;
            }
            Message::ZoomDelta(_pos, _bounds, delta) => {
                let delta = delta * ZOOM_WHEEL_SCALE;
                // let prev_scale = self.graph.controls.scale();
                let prev_zoom = self.graph.controls.zoom;
                self.graph.controls.zoom = prev_zoom + delta;

                // let vec = pos - dvec2(bounds.width.into(), bounds.height.into()) * 0.5;
                // let new_scale = self.graph.controls.scale();
                // self.graph.controls.center += vec * (prev_scale - new_scale) * 2.0;
            }
            Message::SetError(err_msg) => self.err_msg = Some(err_msg),
            Message::ClearErrors => self.err_msg = None,
        }
    }
}

impl MainState {
    fn view(&'_ self) -> Element<'_, Message> {
        column![
            container(text("Title").size(30)).center_x(Fill),
            row![
                column![].width(50),
                self.text_editor_view(),
                vertical_space(),
                self.graph_view(),
                column![].width(50)
            ]
        ]
        .height(Fill)
        .into()
    }

    fn text_editor_view(&'_ self) -> Element<'_, Message> {
        column![
            text("Editor").size(30).height(FillPortion(6)),
            container(
                widget::text_editor(&self.text)
                    .placeholder("f(x) = (-x)**3 + 1")
                    .size(30)
                    .height(Fill)
                    .on_action(Message::EditText)
            )
            .width(FillPortion(30))
            .height(FillPortion(90))
            .style(container::rounded_box),
            row![widget::text(
                self.err_msg.clone().unwrap_or("No errors".to_string())
            )]
            .height(FillPortion(20)),
            row![].height(FillPortion(4)),
        ]
        .into()
    }

    fn graph_view(&'_ self) -> Element<'_, Message> {
        column![
            container(widget::shader(&self.graph).height(Fill).width(Fill))
                .style(container::rounded_box)
                .height(Fill)
                .width(FillPortion(70)),
        ]
        .into()
    }
}
