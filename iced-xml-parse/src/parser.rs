use std::{
    io::Seek,
    path::{Path, PathBuf},
};

use iced_xml_core::{
    ButtonNode, ColumnNode, ComponentNode, HorizontalAlignment, Node, NodeLayout, Padding, RowNode,
    TextNode, VerticalAlignment, WindowNode,
};

fn parse_window<'a>(document: roxmltree::Document<'a>) -> WindowNode {
    let root = document.root();
    let window = root.first_element_child().unwrap();
    assert!(window.tag_name().name() == "Window");
    let message_type = window.attribute("Message").unwrap_or("").to_string();

    let content = window
        .first_element_child()
        .map(|node| parse_component(node));

    WindowNode {
        content,
        message_type,
    }
}

fn parse_component<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> ComponentNode {
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
    let content = node.text().unwrap_or("");
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
        .map(|c| Box::new(parse_component(c)))
        .collect::<Vec<_>>();

    ColumnNode { content }
}

fn parse_row_node<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> RowNode {
    let content = node
        .descendants()
        .filter(|d| d.is_element())
        .skip(1)
        .map(|c| Box::new(parse_component(c)))
        .collect::<Vec<_>>();

    RowNode { content }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> WindowNode {
    let content = std::fs::read_to_string(path).unwrap();
    let document = roxmltree::Document::parse(&content).unwrap();
    parse_window(document)
}

pub fn parse_str(content: &str) -> WindowNode {
    let document = roxmltree::Document::parse(content).unwrap();
    parse_window(document)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_empty_window() {
        let raw_window = r#"<?xml version="1.0" encoding="UTF-8"?> 
<Window Message="Message">
</Window>
        "#;

        let parsed = parse_str(raw_window);
    }

    #[test]
    fn can_parse_simple_window() {
        let raw_window = r#"<?xml version="1.0" encoding="UTF-8"?> 
<Window Message="Message">
  <Column Padding="20" AlignX="Center">
    <Button Message="Increment">Increment</Button>
    <Text Size="50" Content="{self.value}" />
    <Button Message="Decrement">Decrement</Button>
  </Column>
</Window>
        "#;

        let parsed = parse_str(raw_window);
    }
}
