use std::{
    io::Seek,
    path::{Path, PathBuf},
};

fn parse_window<'a>(document: roxmltree::Document<'a>) -> ParsedWindow {
    let root = document.root();
    let window = root.first_element_child().unwrap();
    assert!(window.tag_name().name() == "Window");

    let control = window.first_element_child().map(|node| parse_control(node));

    ParsedWindow { control }
}

fn parse_control<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> ParsedControl {
    ParsedControl::Button(parse_button(node))
}

fn parse_button<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> ParsedButton {
    let horizontal_alignment = node
        .attribute("HorizontalAlignment")
        .map(|attr| attr.into())
        .unwrap_or(HorizontalAlignment::default());

    let vertical_alignment = node
        .attribute("VerticalAlignment")
        .map(|attr| attr.into())
        .unwrap_or(VerticalAlignment::default());

    let content = node.text().unwrap_or("");

    ParsedButton {
        content: content.to_string(),
        horizontal_alignment,
        vertical_alignment,
    }
}

pub fn parse<P: AsRef<Path>>(path: P) -> ParsedWindow {
    let content = std::fs::read_to_string(path).unwrap();
    let document = roxmltree::Document::parse(&content).unwrap();
    parse_window(document)
}

#[derive(Debug, Clone)]
pub struct ParsedWindow {
    pub(crate) control: Option<ParsedControl>,
}

#[derive(Debug, Clone)]
pub enum ParsedControl {
    Button(ParsedButton),
}

pub struct ParsedLabel {
    pub(crate) content: String,
}

pub struct ParsedRow {
    pub(crate) controls: Vec<Box<ParsedControl>>,
}

pub struct ParsedColumn {
    pub(crate) controls: Vec<Box<ParsedControl>>,
}

// TODO: Add some notion of a command. How
// would that even work with iced? It would need
// to create a message, but how?
#[derive(Debug, Clone)]
pub struct ParsedButton {
    pub(crate) content: String, // TODO: impl Display?
    pub(crate) horizontal_alignment: HorizontalAlignment,
    pub(crate) vertical_alignment: VerticalAlignment,
}

#[derive(Debug, Copy, Clone)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Stretch,
}
impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self::Left
    }
}
impl<'a> From<&str> for HorizontalAlignment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "left" => HorizontalAlignment::Left,
            "center" => HorizontalAlignment::Center,
            "right" => HorizontalAlignment::Right,
            "stretch" => HorizontalAlignment::Stretch,
            _ => HorizontalAlignment::default(),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Stretch,
}
impl Default for VerticalAlignment {
    fn default() -> Self {
        Self::Top
    }
}
impl<'a> From<&str> for VerticalAlignment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "top" => VerticalAlignment::Top,
            "center" => VerticalAlignment::Center,
            "bottom" => VerticalAlignment::Bottom,
            "stretch" => VerticalAlignment::Stretch,
            _ => VerticalAlignment::default(),
        }
    }
}
