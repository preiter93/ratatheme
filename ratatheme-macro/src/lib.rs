#![allow(dead_code, unused_variables)]
use core::panic;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, Ident, Meta, Type};

/// A procedural macro that parses style attributes, converting color
/// strings into styles with `ratatui::style::Color` values.
///
/// # Panics
/// This macro will panic in the following cases:
/// - The macro is not attached to a struct.
/// - The struct has no named fields to which the macro can attach.
/// - The attribute tag is malformed or incorrect. The expected format is: `#[theme(style)]`.
#[proc_macro_derive(Theme, attributes(theme))]
pub fn theme_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("Theme must have named fields");
    };

    if fields.named.is_empty() {
        panic!("Theme must have one or more fields");
    }

    let mut style_implementation = quote! {};

    let mut has_colors_field = false;

    for field in &fields.named {
        let Some(attr) = find_attribute(&field.attrs) else {
            continue;
        };

        let Some(meta) = find_metadata(attr) else {
            continue;
        };

        match &meta {
            Metadata::Style => {
                let field_name = field.ident.as_ref().unwrap();
                let generated = generate_style_method(field_name);
                style_implementation.extend(generated);
            }
            Metadata::Colors => {
                has_colors_field = true;
            }
            Metadata::Styles(subfields) => {
                let generated = generate_styles_method(&field, subfields);
                style_implementation.extend(generated);
            }
        }
    }

    let colors_implementation = if has_colors_field {
        quote! {
            fn _color_from_str(&self, color: &str) -> ratatui::style::Color {
                use std::str::FromStr;

                let color = self.colors.get(color).unwrap_or(color);
                ratatui::style::Color::from_str(color).expect("failed to parse color from str")
            }
        }
    } else {
        quote! {
            fn _color_from_str(&self, color: &str) -> ratatui::style::Color {
                ratatui::style::Color::from_str(color).expect("failed to parse color from str")
            }
        }
    };

    let expanded = quote! {
        impl #struct_name {
            #style_implementation

            #colors_implementation
        }
    };

    TokenStream::from(expanded)
}

/// Helper to find the `theme` attribute in a list of attributes.
fn find_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("theme"))
}

/// Represents all allowed proc-macro metadata annotations.
enum Metadata {
    Style,
    Colors,
    Styles(Vec<Ident>),
}

/// Helper to find the theme `Metadata` of an attribute.
fn find_metadata(attr: &Attribute) -> Option<Metadata> {
    let Meta::List(list) = &attr.meta else {
        panic!("expected metadata in the format 'theme(style)'");
    };

    let mut iter = list.tokens.clone().into_iter();

    let Some(ident) = iter.next() else {
        return None;
    };

    match ident.to_string().as_str() {
        "style" => Some(Metadata::Style),
        "colors" => Some(Metadata::Colors),
        "styles" => {
            let Some(TokenTree::Group(group)) = iter.next() else {
                panic!("cannot parse styles annotation")
            };

            let mut fields = Vec::new();

            for field in group.stream() {
                if let TokenTree::Ident(ident) = field {
                    fields.push(ident);
                }
            }

            Some(Metadata::Styles(fields))
        }
        s => panic!("expected metadata [style, colors, styles], but got: {s}"),
    }
}

fn generate_style_method(field_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        pub fn #field_name(&self) -> ratatui::style::Style {
            let mut style = ratatui::style::Style::default();

            if let Some(fg) = &self.#field_name.foreground {
                style = style.fg(self._color_from_str(fg));
            }

            if let Some(bg) = &self.#field_name.background {
                style = style.bg(self._color_from_str(bg));
            }

            style
        }
    }
}

fn generate_styles_method(field: &Field, subfields: &[Ident]) -> proc_macro2::TokenStream {
    let Type::Path(path) = &field.ty else {
        panic!("styles type must be of type 'Path'");
    };
    let token_stream = path.into_token_stream();

    let Some(type_ident) = path.path.get_ident() else {
        panic!("failed to get ident of styles type");
    };

    let field_name = field.ident.as_ref().unwrap();

    let mut implementation = quote! {};
    for subfield_name in subfields {
        let function_name_str = format!("{field_name}_{subfield_name}");
        let function_name = Ident::new(&function_name_str, Span::call_site());

        implementation.extend(quote! {
            pub fn #function_name(&self) -> ratatui::style::Style {
                let mut style = ratatui::style::Style::default();

                if let Some(fg) = &self.#field_name.#subfield_name.foreground {
                    style = style.fg(self._color_from_str(fg));
                }

                if let Some(bg) = &self.#field_name.#subfield_name.background {
                    style = style.bg(self._color_from_str(bg));
                }

                style
            }
        });
    }

    implementation
}
