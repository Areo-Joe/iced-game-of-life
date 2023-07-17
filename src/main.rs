use iced::widget::{column, container, Text};

use iced::{executor, Application, Color, Length, Settings, Theme};

fn main() -> iced::Result {
    GameOfLife::run(Settings::default())
}

struct GameOfLife;

impl Application for GameOfLife {
    type Message = ();
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (GameOfLife, iced::Command::none())
    }

    fn title(&self) -> String {
        "Game of Life".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        container(column![Text::new("joe")
            .size(40)
            .style(Color::from([0.0, 0.0, 1.0]))])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
