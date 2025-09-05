use std::{any::Any, ops::Deref};
mod view;

use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{DataStruct, DeriveInput, Fields, Result, parse_macro_input, spanned::Spanned};

fn derive_component_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let source_attr = input
        .attrs
        .iter()
        .filter(|attr| {
            attr.path()
                .get_ident()
                .is_some_and(|path| path.to_string() == "source".to_string())
        })
        .next();
    if source_attr.is_none() {
        return Err(syn::Error::new(input.span(), "Missing 'source' attriute"));
    }

    let component = parse_template_attribute(source_attr.unwrap())?;

    match input.data {
        syn::Data::Struct(data_struct) => {
            Ok(derive_component_struct(input.ident, data_struct, component))
        }
        syn::Data::Enum(data_enum) => todo!(),
        syn::Data::Union(data_union) => todo!(),
    }
}

fn derive_component_struct(
    name: syn::Ident,
    data: DataStruct,
    component: iced_xml_core::Component,
) -> proc_macro2::TokenStream {
    let message = match component.message_type {
        Some(ty) => ty,
        None => "()".to_string(),
    };
    let message = syn::parse_str::<syn::Path>(&message).unwrap();
    let view = view::view_component_node(&component.content.unwrap());

    quote!(
    impl ::iced_xml_core::IcedComponent for #name {
        type Message = crate::#message;
        fn view(&self) -> ::iced::Element<'_, Self::Message> {
            #view
            .into()
        }
    }
        )
}

#[proc_macro_derive(IcedComponent, attributes(source))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_component_impl(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[cfg(test)]
pub fn derive_component_test(input: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream> {
    let item = syn::parse2::<DeriveInput>(input)?;

    derive_component_impl(item)
}

fn parse_template_attribute(attr: &syn::Attribute) -> Result<iced_xml_core::Component> {
    // This would work in rust 1.88+ only
    // if let syn::Meta::NameValue(path) = attr.meta
    //     && let syn::Expr::Lit(path) = path.value
    //     && let syn::Lit::Str(path) = path.lit
    // {
    //     Some(iced_xml_parse::parse_file(path.value()))
    // } else {
    //     None
    // }
    //

    // Unstable, so can not use span to get source file :(
    // let path_span = proc_macro::Span::call_site();
    // let source_file = path_span.type_id()

    let toml_dir =
        std::path::Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("src");

    let error = syn::Error::new(attr.span(), "Invalid syntax");

    if let syn::Meta::NameValue(path) = attr.meta.clone() {
        if let syn::Expr::Lit(path) = path.value {
            if let syn::Lit::Str(path) = path.lit {
                let path = toml_dir.join(path.value());
                if let Ok(exists) = std::fs::exists(&path) {
                    if !exists {
                        Err(syn::Error::new(
                            attr.span(),
                            format!("could not find component file '{path:?}'"),
                        ))
                    } else {
                        Ok(iced_xml_parse::parse_file(path))
                    }
                } else {
                    Err(syn::Error::new(
                        attr.span(),
                        format!("could not find component file '{path:?}'"),
                    ))
                }
            } else {
                Err(error)
            }
        } else {
            Err(error)
        }
    } else {
        Err(error)
    }
}

// fn handle_data_struct(data: &DataStruct) -> TokenStream {
//     if let Fields::Named(ref fields) = data.fields {};
// }

#[proc_macro_attribute]
pub fn template(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(test)]
pub(crate) fn prettyprint(tokens: proc_macro2::TokenStream) -> String {
    use syn::parse::Parse;

    // prettyplease::unparse(&syn::parse_str::<syn::File>(stringify!(tokens)).unwrap())

    stringify!(tokens).to_string()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn struct_without_component_attr_complains() {
        let input = quote! {
            #[derive(Component)]
            struct Test;
        };

        derive_component_test(input.into());
    }
}
