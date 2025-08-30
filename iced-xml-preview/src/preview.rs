use iced::{
    Element, alignment,
    widget::{Container, button, column, container, text},
};
use iced_xml_core::NodeLayout;
#[derive(Debug, Copy, Clone)]
pub(crate) struct DummyMessage;
pub(crate) trait ComponentNodePreview {
    fn view(&self) -> iced::Element<'_, DummyMessage>;
}
pub(crate) trait TypedNodePreview {
    fn view(&self, layout: iced_xml_core::NodeLayout) -> iced::Element<'_, DummyMessage>;
}

fn layout_container(
    layout: NodeLayout,
    content: iced::Element<'_, DummyMessage>,
) -> Container<'_, DummyMessage> {
    let align_x = match layout.align_x {
        iced_xml_core::HorizontalAlignment::Left => alignment::Horizontal::Left,
        iced_xml_core::HorizontalAlignment::Center => alignment::Horizontal::Center,
        iced_xml_core::HorizontalAlignment::Right => alignment::Horizontal::Right,
        iced_xml_core::HorizontalAlignment::Stretch => todo!(),
    };

    let align_y = match layout.align_y {
        iced_xml_core::VerticalAlignment::Top => alignment::Vertical::Top,
        iced_xml_core::VerticalAlignment::Center => alignment::Vertical::Center,
        iced_xml_core::VerticalAlignment::Bottom => alignment::Vertical::Bottom,
        iced_xml_core::VerticalAlignment::Stretch => todo!(),
    };

    let padding = iced::Padding::new(layout.padding.value);

    container(content)
        .align_x(align_x)
        .align_y(align_y)
        .padding(padding)
}

impl ComponentNodePreview for iced_xml_core::ComponentNode {
    fn view(&self) -> iced::Element<'_, DummyMessage> {
        match &self.node {
            iced_xml_core::Node::Button(button_node) => button_node.view(self.layout),
            iced_xml_core::Node::Text(text_node) => text_node.view(self.layout),
            iced_xml_core::Node::Column(column_node) => column_node.view(self.layout),
            iced_xml_core::Node::Row(row_node) => row_node.view(self.layout),
        }
    }
}

impl TypedNodePreview for iced_xml_core::ButtonNode {
    fn view(&self, layout: iced_xml_core::NodeLayout) -> iced::Element<'_, DummyMessage> {
        let button = button(text!("{}", self.content)).on_press(DummyMessage);
        layout_container(layout, button.into()).into()
    }
}

impl TypedNodePreview for iced_xml_core::TextNode {
    fn view(&self, layout: iced_xml_core::NodeLayout) -> iced::Element<'_, DummyMessage> {
        let text = text!("{}", self.content).size(self.size);
        layout_container(layout, text.into()).into()
    }
}

impl TypedNodePreview for iced_xml_core::ColumnNode {
    fn view(&self, layout: iced_xml_core::NodeLayout) -> iced::Element<'_, DummyMessage> {
        let children = self.content.iter().map(|c| c.view());
        let column = column(children);
        layout_container(layout, column.into()).into()
    }
}

impl TypedNodePreview for iced_xml_core::RowNode {
    fn view(&self, layout: iced_xml_core::NodeLayout) -> iced::Element<'_, DummyMessage> {
        let children = self.content.iter().map(|c| c.view());
        let column = column(children);
        layout_container(layout, column.into()).into()
    }
}
