use iced::{
    Length,
    widget::{button, center, container, text},
};

use crate::parser::ParsedButton;

pub trait Control<M: Clone> {
    fn view(&self) -> iced::Element<'_, M>;
}

// FIXME: Does not work since M does not live long enought
// impl<M: std::clone::Clone> Control<M> for ParsedButton {
// fn view(&self) -> iced::Element<'_, M> {
//     let make_button = || button(text!("{}", self.content));
//     match self.horizontal_alignment {
//         crate::parser::HorizontalAlignment::Center => {
//             container(make_button()).center(Length::Fill).into()
//         }
//         _ => make_button().into(),
//     }
// }
// }
