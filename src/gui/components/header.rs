use crate::gui::types::message::Message;
use iced::widget::{Container, Row, Text};

pub fn header<'a>() -> Container<'a, Message> {
    Container::new(Row::new().push(Text::new("Default header".to_string()))).height(45)
}
