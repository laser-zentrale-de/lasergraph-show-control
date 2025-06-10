use crate::gui::types::message::Message;
use iced::widget::{Container, Row, Text};

pub fn footer<'a>() -> Container<'a, Message> {
    Container::new(Row::new().push(Text::new("Default footer".to_string()))).height(45)
}
