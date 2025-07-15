use iced::{
    Element,
    Length::Fill,
    Pixels, Theme,
    widget::{column, container, row, text, text_editor},
};

fn main() -> iced::Result {
    iced::application("MathLang", MainState::update, MainState::view)
        .theme(|_| Theme::SolarizedDark)
        .run()
}

#[derive(Debug, Default)]
pub struct MainState {
    text: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditText(text_editor::Action),
}

impl MainState {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::EditText(action) => self.text.perform(action),
        }
    }
}

impl MainState {
    fn view(&'_ self) -> Element<'_, Message> {
        column![
            container(text("Title").size(30)).center_x(Fill),
            row![column![
                text("Editor").size(20),
                container(
                    text_editor(&self.text)
                        .placeholder("Enter equation..")
                        .size(20)
                        .on_action(Message::EditText)
                        .height(Fill)
                        .width(Pixels(500.0))
                )
                .style(container::rounded_box),
            ]]
        ]
        .height(Fill)
        .into()
    }
}
