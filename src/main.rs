use gui::app::App;
#[cfg(target_os = "linux")]
use iced::window::settings::PlatformSpecific;
use iced::{application, window};

mod gui;

const APP_TITLE_LOWER: &str = "lasergraph-show-control";

fn main() -> iced::Result {
    application(APP_TITLE_LOWER, App::update, App::view)
        .theme(App::theme)
        .window(window::Settings {
            size: iced::Size::new(600.0, 350.0),
            position: window::Position::Default,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: true,
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: String::from(APP_TITLE_LOWER),
                ..PlatformSpecific::default()
            },
            exit_on_close_request: false,
            ..Default::default()
        })
        .run()
}
