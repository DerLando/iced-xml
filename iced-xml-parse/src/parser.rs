use std::{
    io::Seek,
    path::{Path, PathBuf},
};

use iced_xml_core::{
    ButtonNode, ColumnNode, Component, ComponentNode, HorizontalAlignment, Node, NodeLayout,
    Padding, RowNode, TextNode, VerticalAlignment,
};

fn parse_component<'a>(document: roxmltree::Document<'a>) -> Component {
    let root = document.root();
    let window = root.first_element_child().unwrap();
    assert!(window.tag_name().name() == "Component");
    let message_type = window.attribute("Message").map(|attr| attr.to_string());
    let model_type = window.attribute("Model").map(|attr| attr.to_string());

    let content = window
        .first_element_child()
        .map(|node| parse_component_node(node));

    Component {
        content,
        message_type,
        model_type,
    }
}

fn parse_component_node<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> ComponentNode {
    let layout = parse_layout(&node);
    let component = match node.tag_name().name() {
        "Button" => Node::Button(parse_button_node(node)),
        "Text" => Node::Text(parse_text_node(node)),
        "Column" => Node::Column(parse_column_node(node)),
        "Row" => Node::Row(parse_row_node(node)),
        _ => todo!(), //TODO: Especially custom components will be interesting...
    };

    ComponentNode {
        node: component,
        layout,
    }
}

fn parse_layout<'a, 'input>(node: &roxmltree::Node<'a, 'input>) -> NodeLayout {
    let horizontal_alignment = node
        .attribute("HorizontalAlignment")
        .map(|attr| attr.into())
        .unwrap_or(HorizontalAlignment::default());

    let vertical_alignment = node
        .attribute("VerticalAlignment")
        .map(|attr| attr.into())
        .unwrap_or(VerticalAlignment::default());

    let padding = node
        .attribute("Padding")
        .map(|attr| attr.parse::<f32>().unwrap_or(0.0))
        .map(|p| Padding { value: p })
        .unwrap_or(Padding::default());

    NodeLayout {
        align_x: horizontal_alignment,
        align_y: vertical_alignment,
        padding,
    }
}

fn parse_button_node<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> ButtonNode {
    let content = node.text().unwrap_or("");
    let message = node.attribute("Message").unwrap_or("");

    ButtonNode {
        content: content.to_string(),
        message: message.to_string(),
    }
}

fn parse_text_node<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> TextNode {
    const DEFAULT_SIZE: f32 = 10.0;
    let content = match node.text() {
        Some(text) => text,
        None => node.attribute("Content").unwrap_or(""),
    };
    let size = node
        .attribute("Size")
        .map(|s| s.parse::<f32>().unwrap_or(DEFAULT_SIZE))
        .map(|s| s)
        .unwrap_or(DEFAULT_SIZE);

    TextNode {
        content: content.to_string(),
        size,
    }
}

fn parse_column_node<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> ColumnNode {
    let content = node
        .descendants()
        .filter(|d| d.is_element())
        .skip(1)
        .map(|c| Box::new(parse_component_node(c)))
        .collect::<Vec<_>>();

    ColumnNode { content }
}

fn parse_row_node<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> RowNode {
    let content = node
        .descendants()
        .filter(|d| d.is_element())
        .skip(1)
        .map(|c| Box::new(parse_component_node(c)))
        .collect::<Vec<_>>();

    RowNode { content }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Component {
    let content = std::fs::read_to_string(path).unwrap();
    let document = roxmltree::Document::parse(&content).unwrap();
    parse_component(document)
}

pub fn parse_str(content: &str) -> Component {
    let document = roxmltree::Document::parse(content).unwrap();
    parse_component(document)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_empty_component() {
        let raw_window = r#"<?xml version="1.0" encoding="UTF-8"?> 
<Component Message="Message">
</Component>
        "#;

        let parsed = parse_str(raw_window);
    }

    #[test]
    fn can_parse_simple_window() {
        let raw_window = r#"<?xml version="1.0" encoding="UTF-8"?> 
<Component Message="Message">
  <Column Padding="20" AlignX="Center">
    <Button Message="Increment">Increment</Button>
    <Text Size="50" Content="{self.value}" />
    <Button Message="Decrement">Decrement</Button>
  </Column>
</Component>
        "#;

        let parsed = parse_str(raw_window);
    }

    #[test]
    fn can_parse_button_node_content() {
        let raw_button = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Button>Hello</Button>
        "#;

        let tree = roxmltree::Document::parse(raw_button).unwrap();
        let parsed_button = parse_button_node(tree.root_element());

        assert_eq!(parsed_button.content, "Hello".to_string());
    }
}
