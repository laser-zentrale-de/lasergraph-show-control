//! This module defines the application GUI.

use crate::gui::types::message::Message;

/// App struct contains the main application gui.
///
/// The fields control the app state.
#[derive(Default)]
pub struct App {}

impl App {
    /// Handle App state mutations based on the message passed
    pub fn update(&mut self, _message: Message) {}

    /// Returns the UI of the application
    pub fn view(&self) -> iced::Element<Message> {
        "Here will be the lasergraph show control".into()
    }

    /// Returns the theme of the application
    pub fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
}
