use iced::{
    Element,
    Length::{Fill, FillPortion},
    widget::{self, column, container, row, space, text},
};

use crate::{MainState, message::Message};

impl MainState {
    pub fn view(&'_ self) -> Element<'_, Message> {
        row![
            container(self.text_editor_view()).width(FillPortion(30)),
            container(self.graph_view()).width(FillPortion(70))
        ]
        .padding(50)
        .into()
    }

    fn text_editor_view(&'_ self) -> Element<'_, Message> {
        column![
            container(
                widget::text_editor(&self.text)
                    .size(30)
                    .height(Fill)
                    .on_action(Message::EditText)
            )
            .height(FillPortion(80))
            .style(container::rounded_box),
            container(widget::text(
                self.err_msg.clone().unwrap_or("No errors".to_string())
            ))
            .height(FillPortion(20)),
        ]
        .into()
    }

    fn graph_view(&'_ self) -> Element<'_, Message> {
        widget::shader(&self.graph).height(Fill).width(Fill).into()
    }
}
