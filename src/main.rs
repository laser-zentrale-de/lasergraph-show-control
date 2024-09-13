use iced::{
    alignment::Horizontal,
    executor, theme,
    widget::{button, column, row, text},
    Application, Command, Length, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    ButtonDmxLoad,
    ButtonDmxStart,
    ButtonDmxStop,
    ButtonTimecodeLoad,
    ButtonTimecodeStart,
    ButtonTimecodeStop,
}

struct MyApp {
    state: String,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Theme = iced::Theme;
    type Flags = ();
    type Message = MyAppMessage;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                state: "No State".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Lasergraph Show Control")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::ButtonDmxLoad => self.state = "DMX Load".to_string(),
            MyAppMessage::ButtonDmxStart => self.state = "DMX Start".to_string(),
            MyAppMessage::ButtonDmxStop => self.state = "DMX Stop".to_string(),
            MyAppMessage::ButtonTimecodeLoad => self.state = "Timecode Load".to_string(),
            MyAppMessage::ButtonTimecodeStart => self.state = "Timecode Start".to_string(),
            MyAppMessage::ButtonTimecodeStop => self.state = "Timecode Stop".to_string(),
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            text(self.state.clone())
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            row![
                button("DMX Load")
                    .style(theme::Button::Primary)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(MyAppMessage::ButtonDmxLoad)
                    .padding(20),
                button("DMX Start")
                    .style(theme::Button::Positive)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(MyAppMessage::ButtonDmxStart)
                    .padding(20),
                button("DMX Stop")
                    .style(theme::Button::Secondary)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(MyAppMessage::ButtonDmxStop)
                    .padding(20),
            ]
            .spacing(20)
            .padding(20),
            row![
                button("Timecode Load")
                    .style(theme::Button::Primary)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(MyAppMessage::ButtonTimecodeLoad)
                    .padding(20),
                button("Timecode Start")
                    .style(theme::Button::Positive)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(MyAppMessage::ButtonTimecodeStart)
                    .padding(20),
                button("Timecode Stop")
                    .style(theme::Button::Secondary)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(MyAppMessage::ButtonTimecodeStop)
                    .padding(20),
            ]
            .spacing(20)
            .padding(20)
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
