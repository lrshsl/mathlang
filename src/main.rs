use glam::{DVec2, Vec2};
use iced::{
    Element,
    Length::{Fill, FillPortion},
    Rectangle, Theme,
    widget::{self, column, container, row, text, text_editor, vertical_space},
};

mod parser;
use parser::parse_func;

mod graph;
use graph::Graph;

mod ops;
mod ast;

use crate::ops::Op;

pub const ZOOM_DEFAULT: f64 = 2.0;
pub const ZOOM_WHEEL_SCALE: f64 = 0.2;

fn main() -> iced::Result {
    iced::application("MathLang", MainState::update, MainState::view)
        .theme(|_| Theme::SolarizedDark)
        .run()
}

#[derive(Default)]
pub struct MainState {
    text: text_editor::Content,
    graph: Graph,
    program: Vec<Op>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditText(text_editor::Action),
    UpdateOp(usize, u32, f32),
    PanningDelta(DVec2),
    UpdateZoom(f64),
    ZoomDelta(DVec2, Rectangle, f64),
}

impl MainState {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::EditText(action) => {
                self.text.perform(action);
                if let Ok((_name, fun)) = parse_func(&self.text.text()) {
                    self.program.push(Func::new(new, fun))
                }
            }
            Message::UpdateOp(i, opcode, operand) => {
                if let Some(f) = self.graph.controls.program.get_mut(i) {
                    *f = Op {
                        opcode,
                        operand,
                        _pad: Vec2::ZERO,
                    }
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
                text_editor(&self.text)
                    .placeholder("Enter equation..")
                    .size(30)
                    .height(Fill)
                    .on_action(Message::EditText)
            )
            .width(FillPortion(30))
            .height(FillPortion(90))
            .style(container::rounded_box),
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
