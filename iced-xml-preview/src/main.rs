use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use iced::{Element, Subscription, Task, widget::text};
use preview::ComponentNodePreview;
mod preview;

const DEFAULT_IXML: &str = include_str!("./hello_world.ixml");

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    let initial_path = match &args[..] {
        [_, path] => PathBuf::from_str(&path).ok(),
        _ => None,
    };
    iced::application("IXML-Previewer", App::update, App::view)
        .subscription(App::subscription)
        .run_with(|| (App::with_initial_path(initial_path), Task::none()))
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    PathChanged(PathBuf),
    NOP,
}

#[derive(Debug, Default)]
struct App {
    parsed: Option<iced_xml_core::WindowNode>,
    path: Option<PathBuf>,
}

impl App {
    pub fn with_initial_path<P: AsRef<Path>>(path: Option<P>) -> Self {
        Self {
            parsed: None,
            path: path.map(|p| p.as_ref().into()),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(iced::time::Duration::from_millis(500)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.parsed = match &self.path {
                    None => Some(iced_xml_parse::parse_str(DEFAULT_IXML)),
                    Some(path) => Some(iced_xml_parse::parse_file(path)),
                }
            }
            Message::PathChanged(path) => self.path = Some(path),
            Message::NOP => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.parsed {
            None => text!("Invalid xml").into(),
            Some(parsed) => match &parsed.content {
                None => text!("").into(),
                Some(content) => content.view().map(|_| Message::NOP),
            },
        }
    }
}
