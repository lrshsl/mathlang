use glam::Vec2;
use iced::{
    Element,
    Length::{Fill, FillPortion},
    Pixels, Rectangle, Theme,
    widget::{
        self, column, container, row,
        text, text_editor, vertical_space,
    },
};

mod parser;
use parser::parse_func;

mod graph;
use graph::Graph;

fn main() -> iced::Result {
    iced::application("MathLang", MainState::update, MainState::view)
        .theme(|_| Theme::SolarizedDark)
        .run()
}

#[derive(Default)]
pub struct MainState {
    text: text_editor::Content,
    objects: Vec<Box<dyn FnOnce(f64) -> f64>>,
    graph: Graph,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditText(text_editor::Action),
    ZoomDelta(Vec2, Rectangle, f32),
}

impl MainState {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::EditText(action) => {
                self.text.perform(action);
                if let Ok((_name, fun)) = parse_func(&self.text.text()) {
                    self.objects.push(fun)
                }
            }
            Message::ZoomDelta(_vec2, _rectangle, _factor) => todo!(),
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
            .width(Pixels(500.0))
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
                .width(Fill),
        ]
        .into()
    }
}
