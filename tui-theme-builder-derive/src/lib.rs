use core::panic;
use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident};

/// # Panics
/// - Panics if derive is not attached to a struct
/// - Panics if no `context` attribute is found
#[proc_macro_derive(ThemeBuilder, attributes(context, builder, style))]
pub fn derive_theme_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let builder_attr = extract_builder_attribute(&input.attrs);
    let Some(builder_attr) = builder_attr else {
        panic!("no `context` attribute found on struct");
    };
    let context_name = process_builder_struct_attribute(builder_attr);
    let Some(context_name) = context_name else {
        panic!("no `context` field found in builder annotation");
    };

    let Fields::Named(fields) = &data.fields else {
        panic!("expected named fields, got {:?}", &data.fields)
    };

    let mut field_constructors: Vec<TokenStream2> = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let mut field_constructor = quote! {};

        // Handle `Style` tagged fields.
        let attr = extract_style_attribute(&field.attrs);
        if let Some(attr) = attr {
            let style_values = process_style_attribute(attr);

            field_constructor.extend(quote! {
                #field_name: ratatui::style::Style::default()
            });

            if let Some(foreground_color) = style_values.foreground {
                field_constructor.extend(quote! {
                    .fg(context.#foreground_color.clone().into())
                });
            }

            if let Some(background_color) = style_values.background {
                field_constructor.extend(quote! {
                    .bg(context.#background_color.clone().into())
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

            field_constructors.push(field_constructor);
            continue;
        }

        // Handle `builder` tagged fields.
        let attr = extract_builder_attribute(&field.attrs);
        if let Some(attr) = attr {
            let value = process_builder_field_attribute(attr);
            let Some(value) = value else {
                panic!("missing value in `builder` on field `{:?}`", field_name);
            };

            field_constructor.extend(quote! {
                    #field_name: context.#value.clone()
            });

            field_constructors.push(field_constructor);
            continue;
        }

        // Handle untagged fields.
        field_constructor.extend(quote! {
                #field_name: <#field_type>::default()
        });

        field_constructors.push(field_constructor);
    }

    let implementation = quote! {
        impl tui_theme_builder::ThemeBuilder for #struct_name {
            type Context = #context_name;
            fn build(context: &#context_name) -> Self {
                Self {
                    #(#field_constructors),*
                }
            }
        }
    };

    TokenStream::from(implementation)
}

/// A helper method to extract the `builder` attribute in a list of attributes.
fn extract_builder_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("builder"))
}

/// A helper method that processes a field with builder annotation.
fn process_builder_field_attribute(attr: &Attribute) -> Option<TokenStream2> {
    let mut context: Option<TokenStream2> = None;

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("value") {
            let value = meta.value()?;
            let value = parse_stream_to_token_stream(&value)?;
            context = Some(value);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    });

    context
}

/// A helper method that parses a `ParseStream` to a `TokenStream`. It is necessary
/// to handle nested fields such as `#[builder(value=footer.hide)]`
fn parse_stream_to_token_stream(input: ParseStream) -> Result<TokenStream2, syn::Error> {
    let mut tokens = TokenStream2::new();
    while !input.is_empty() {
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            tokens.extend(Some(TokenTree::Ident(ident)));
        } else if input.peek(syn::Token![.]) {
            let _dot: syn::Token![.] = input.parse()?;
            tokens.extend(Some(TokenTree::Punct(Punct::new('.', Spacing::Alone))));
        } else {
            return Err(input.error("expected an identifier or a dot"));
        }
    }

    Ok(tokens)
}

/// Helper to that process the builder attribute of a struct and returns the
/// ident of the context type.
fn process_builder_struct_attribute(attr: &Attribute) -> Option<Ident> {
    let mut context: Option<Ident> = None;

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("context") {
            let value = meta.value()?;
            let ident: syn::Ident = value.parse()?;
            context = Some(ident);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    });

    context
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
                "fg" | "foreground" => {
                    let value = meta.value()?;
                    let ident = value.parse()?;
                    foreground = Some(ident);
                }
                "bg" | "background" => {
                    let value = meta.value()?;
                    let ident = value.parse()?;
                    background = Some(ident);
                }
                _ => {}
            };
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
