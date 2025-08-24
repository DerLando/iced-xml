use std::env::current_dir;

use iced::{
    Element, Length, Subscription,
    widget::{button, container, text},
};
use parser::{ParsedButton, ParsedWindow, parse};

mod control;
mod parser;
fn main() -> iced::Result {
    iced::application("IXML", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

#[derive(Debug, Default)]
struct App {
    parsed: Option<ParsedWindow>,
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(iced::time::Duration::from_millis(500)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => self.parsed = Some(parser::parse("./src/main_window.ixml")),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.parsed {
            None => text!("Invalid xml").into(),
            Some(parsed) => match &parsed.control {
                Some(control) => match control {
                    parser::ParsedControl::Button(parsed_button) => {
                        Self::view_button(&parsed_button)
                    }
                },
                None => todo!(),
            },
        }
    }

    // TODO: Figure out how to move this into a trait
    fn view_button(btn: &ParsedButton) -> Element<'_, Message> {
        let make_button = || button(text!("{}", btn.content));
        // TODO: Proper alignment support
        match btn.horizontal_alignment {
            crate::parser::HorizontalAlignment::Center => {
                container(make_button()).center_x(Length::Fill).into()
            }
            _ => make_button().into(),
        }
    }
}
