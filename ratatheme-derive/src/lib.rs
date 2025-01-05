use core::panic;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parenthesized, parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, LitStr};

#[proc_macro_derive(ThemeBuilder, attributes(colors, builder, style))]
pub fn derive_from_colors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let builder_attr = extract_builder_attribute(&input.attrs);
    let Some(builder_attr) = builder_attr else {
        panic!("no `colors` attribute found on struct");
    };
    let colors_name = process_builder_attribute(builder_attr);
    let Some(colors_name) = colors_name else {
        panic!("no `colors` field found in builder annotataion");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("expected named fields, got {:?}", &data.fields)
    };

    let mut field_constructors: Vec<TokenStream2> = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let mut field_constructor = quote! {};

        let attr = extract_style_attribute(&field.attrs);
        if let Some(attr) = attr {
            // Handle tagged fields. They must be of type `Style`.
            let style_values = process_style_attribute(attr);

            field_constructor.extend(quote! {
                #field_name: ratatui::style::Style::default()
            });
            if let Some(foreground_color) = style_values.foreground {
                field_constructor.extend(quote! {
                    .fg(color.#foreground_color.clone().into())
                });
            }
            if let Some(background_color) = style_values.background {
                field_constructor.extend(quote! {
                    .bg(color.#background_color.clone().into())
                });
            }

            if style_values.bold.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::BOLD)
                });
            }

            if style_values.dim.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::DIM)
                });
            }

            if style_values.italic.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::ITALIC)
                });
            }

            if style_values.underlined.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::UNDERLINED)
                });
            }

            if style_values.slow_blink.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::SLOW_BLINK)
                });
            }

            if style_values.rapid_blink.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::RAPID_BLINK)
                });
            }

            if style_values.reversed.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::REVERSED)
                });
            }

            if style_values.hidden.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::HIDDEN)
                });
            }

            if style_values.crossed_out.is_some() {
                field_constructor.extend(quote! {
                    .add_modifier(ratatui::style::Modifier::CROSSED_OUT)
                });
            }
        } else {
            // Handle untagged fields.
            field_constructor.extend(quote! {
                    #field_name: <#field_type>::default()
            });
        }

        field_constructors.push(field_constructor);
    }

    let implementation = quote! {
        impl #struct_name {
            pub fn build(color: #colors_name) -> Self {
                Self {
                    #(#field_constructors),*
                }
            }
        }
    };

    // let implementation = quote! {
    //     impl From<#colors_name> for #struct_name {
    //         fn from(color: #colors_name) -> Self {
    //             Self {
    //                 #(#field_constructors),*
    //             }
    //         }
    //     }
    // };

    TokenStream::from(implementation)
}

/// A helper method to extract the `builder` attribute in a list of attributes.
fn extract_builder_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("builder"))
}

/// Helper to that process the builder attribute of a struct and returns the
/// ident of the color type.
fn process_builder_attribute(attr: &Attribute) -> Option<Ident> {
    let mut color: Option<Ident> = None;

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("colors") {
            let value = meta.value()?;
            let ident: syn::Ident = value.parse()?;
            color = Some(ident);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    });

    color
}

/// A helper method to extract the `style` attribute in a list of attributes.
fn extract_style_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("style"))
}

/// A helper method that processes a field with style annotation.
fn process_style_attribute(attr: &Attribute) -> StyleValues {
    let mut foreground: Option<Ident> = None;
    let mut background: Option<Ident> = None;
    let mut bold: Option<bool> = None;
    let mut dim: Option<bool> = None;
    let mut italic: Option<bool> = None;
    let mut underlined: Option<bool> = None;
    let mut slow_blink: Option<bool> = None;
    let mut rapid_blink: Option<bool> = None;
    let mut reversed: Option<bool> = None;
    let mut hidden: Option<bool> = None;
    let mut crossed_out: Option<bool> = None;

    let _ = attr.parse_nested_meta(|meta| {
        if meta.input.peek(syn::token::Paren) {
            let meta_name = meta.path.get_ident();
            let Some(meta_name) = meta_name else {
                return Err(meta.error("Expected an identifier in the metadata path"));
            };

            let content;
            parenthesized!(content in meta.input);

            // #[style(fg("primary"))]
            let ident = if content.peek(LitStr) {
                let lit: LitStr = content.parse()?;
                format_ident!("{}", lit.value())
            // #[style(fg(primary))]
            } else if content.peek(syn::Ident) {
                let ident: syn::Ident = content.parse()?;
                ident
            } else {
                return Err(meta.error("Expected string or identifier"));
            };

            match meta_name.to_string().as_str() {
                "fg" | "foreground" => foreground = Some(ident),

                "bg" | "background" => background = Some(ident),
                _ => {}
            }
        } else {
            if let Some(ident) = meta.path.get_ident() {
                match ident.to_string().as_str() {
                    "bold" => bold = Some(true),
                    "dim" => dim = Some(true),
                    "italic" => italic = Some(true),
                    "underlined" => underlined = Some(true),
                    "slow_blink" => slow_blink = Some(true),
                    "rapid_blink" => rapid_blink = Some(true),
                    "reversed" => reversed = Some(true),
                    "hidden" => hidden = Some(true),
                    "crossed_out" => crossed_out = Some(true),
                    _ => {}
                };
            }
        }

        Ok(())
    });

    StyleValues {
        foreground,
        background,
        bold,
        dim,
        italic,
        underlined,
        slow_blink,
        rapid_blink,
        reversed,
        hidden,
        crossed_out,
    }
}

struct StyleValues {
    foreground: Option<Ident>,
    background: Option<Ident>,
    bold: Option<bool>,
    dim: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    slow_blink: Option<bool>,
    rapid_blink: Option<bool>,
    reversed: Option<bool>,
    hidden: Option<bool>,
    crossed_out: Option<bool>,
}
