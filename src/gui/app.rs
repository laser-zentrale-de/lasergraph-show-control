//! This module defines the application GUI.

use crate::gui::components::body::body;
use crate::gui::components::footer::footer;
use crate::gui::components::header::header;
use crate::gui::types::message::Message;
use iced::widget::Column;

/// App struct contains the main application gui.
///
/// The fields control the app state.
#[derive(Default)]
pub struct App {
    command: String,
}

impl App {
    /// Handle App state mutations based on the message passed
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowLoadPressed => self.command = "Show Loaded".to_string(),
            Message::ShowStartPressed => self.command = "Show Started".to_string(),
            Message::ShowStopPressed => self.command = "Show Stopped".to_string(),
        }
    }

    /// Returns the UI of the application
    pub fn view(&self) -> iced::Element<Message> {
        Column::new()
            .push(header())
            .push(body())
            .push(footer())
            .into()
    }

    /// Returns the theme of the application
    pub fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
}
