use iced::widget::{button, text};
use iced::Element;
use crate::gui::update::Message;

pub fn view(counter: &u64) -> Element<Message> {
    button(text(counter)).on_press(Message::Increment).into()
}