use iced::{
    Element,
    Length::{Fill, FillPortion},
    widget::{self, column, container, row, space, text},
};

use crate::{MainState, message::Message};

impl MainState {
    pub fn view(&'_ self) -> Element<'_, Message> {
        column![
            container(text("Title").size(30)).center_x(Fill),
            row![
                column![].width(50),
                self.text_editor_view(),
                space().height(Fill),
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
