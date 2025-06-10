use crate::gui::types::message::Message;
use iced::widget::{Button, Column, Container, Row, Text};
use iced::Length;
use iced::{Alignment, Color};

pub fn body<'a>() -> Container<'a, Message> {
    let status_row = Row::new()
        .push(Text::new("Status".to_string()))
        .push(Text::new("DEFAULT".to_string()).color(Color::BLACK))
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(20);

    let timecode_row = Row::new()
        .push(
            Button::new("Timecode Load")
                .on_press(Message::ShowLoadPressed)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .push(
            Button::new("Timecode Start")
                .on_press(Message::ShowStartPressed)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .push(
            Button::new("Timecode Stop")
                .on_press(Message::ShowStopPressed)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .spacing(30)
        .padding(30)
        .align_y(Alignment::Center)
        .height(Length::Fill);

    let body = Column::new()
        .push(status_row)
        .push(timecode_row)
        .align_x(Alignment::Center);

    Container::new(body)
        .height(Length::Fill)
        .align_y(Alignment::Start)
        .align_x(Alignment::Center)
}
