use iced_xml_core::{ButtonNode, ColumnNode, ComponentNode, NodeLayout, TextNode};
use proc_macro2::{Span, TokenStream};
use quote::quote;

fn is_iced_expr(expr: &str) -> bool {
    expr.starts_with("{") && expr.ends_with("}")
}

fn trim_iced_expr(expr: &str) -> &str {
    expr.trim_start_matches("{").trim_end_matches("}")
}

fn parse_expr(expr: &str) -> syn::Result<syn::Expr> {
    if is_iced_expr(expr) {
        syn::parse_str::<syn::Expr>(trim_iced_expr(expr))
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Not a valid iced expr",
        ))
    }
}

fn parse_expr_or_lit(expr_or_lit: &str) -> syn::Result<syn::Expr> {
    if let Ok(parsed) = parse_expr(expr_or_lit) {
        Ok(parsed)
    } else {
        syn::parse_str::<syn::Expr>(format!("\"{expr_or_lit}\"").as_str())
    }
}

pub(crate) fn view_component_node(node: &ComponentNode) -> TokenStream {
    match &node.node {
        iced_xml_core::Node::Button(button_node) => view_button_node(button_node),
        iced_xml_core::Node::Text(text_node) => view_text_node(text_node),
        iced_xml_core::Node::Column(column_node) => view_column_node(column_node),
        iced_xml_core::Node::Row(row_node) => todo!(),
    }
}

// TODO: Layout!
// pub(crate) fn view_layout(layout: NodeLayout) -> TokenStream {}

pub(crate) fn view_button_node(node: &ButtonNode) -> TokenStream {
    let content =
        parse_expr_or_lit(&node.content).unwrap_or(syn::parse_str::<syn::Expr>("\"\"").unwrap());
    let message = format!("on_press(Self::Message::{})", trim_iced_expr(&node.message));
    let message = syn::parse_str::<syn::Expr>(&message).ok();
    let message_call = message.clone().map(|_| syn::Token![.](Span::call_site()));
    quote!(
        ::iced::widget::button(#content)
        #message_call #message
    )
}

pub(crate) fn view_text_node(node: &TextNode) -> TokenStream {
    let content =
        parse_expr_or_lit(&node.content).unwrap_or(syn::parse_str::<syn::Expr>("\"\"").unwrap());
    let size = syn::parse_str::<syn::Expr>(&format!("size({})", node.size)).ok();
    let size_call = if size.is_some() {
        Some(syn::Token![.](Span::call_site()))
    } else {
        None
    };
    quote!(
        ::iced::widget::text(#content)
        #size_call #size
    )
}

pub(crate) fn view_column_node(node: &ColumnNode) -> TokenStream {
    let children = node.content.iter().map(|c| view_component_node(c));

    quote!(::iced::widget::column![
        #(#children),*
    ])
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::prettyprint;

    use super::*;

    #[test]
    fn can_parse_button_content_expr() {
        let node = ButtonNode {
            content: "{self.value}".to_string(),
            message: String::new(),
        };

        let parsed = view_button_node(&node);
        let expected = quote!(iced::widget::button(self.value));

        println!("{}", parsed);

        assert_eq!(prettyprint(parsed), prettyprint(expected));
    }

    #[test]
    fn can_parse_button_content_ident() {
        let node = ButtonNode {
            content: "Click me".to_string(),
            message: String::new(),
        };

        let parsed = view_button_node(&node);
        let expected = quote!(iced::widget::button("Click me"));

        println!("{}", parsed);

        assert_eq!(prettyprint(parsed), prettyprint(expected));
    }

    #[test]
    fn can_parse_button_message() {
        let node = ButtonNode {
            content: "Click me".to_string(),
            message: "{Increment}".to_string(),
        };

        let parsed = view_button_node(&node);
        println!("{parsed}");
        let expected = quote!(iced::widget::button("Click me").message(Incremento));

        println!("{}", parsed);

        assert_eq!(prettyprint(parsed), prettyprint(expected));
        assert!(false);
    }
}
