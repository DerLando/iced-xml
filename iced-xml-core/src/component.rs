pub trait IcedComponent {
    type Message;
    fn view(&self) -> iced::Element<'_, Self::Message>;
}
