use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, Ident, Meta, PathArguments, Type, TypePath,
};

use crate::subtheme::subtheme_proxy_name;

#[allow(clippy::too_many_lines)]
pub(super) fn impl_deserialize_theme(input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let Fields::Named(fields) = &data.fields else {
        return TokenStream::new();
    };

    if fields.named.is_empty() {
        return TokenStream::new();
    }

    let mut field_names: Vec<Ident> = Vec::new();

    let mut declarations = quote! {};
    let mut match_statements = quote! {};
    let mut color_resolution = quote! {};

    for field in &fields.named {
        let meta = handle_field(field);

        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        field_names.push(field_name.clone());

        let type_name = get_type_identifier(&field.ty).unwrap();

        let var_name = match meta {
            Some(Metadata::Style) => {
                let map_name = format_ident!("{field_name}_map");
                declarations.extend(quote! {
                    let mut #map_name: Option<std::collections::HashMap<String, String>> = None;
                });
                map_name
            }
            Some(Metadata::Styles(_)) => {
                let proxy_struct_name = subtheme_proxy_name(&type_name);
                let proxy_var_name = format_ident!("{field_name}_proxy");

                declarations.extend(quote! {
                    let mut #proxy_var_name: Option<#proxy_struct_name> = None;
                });

                proxy_var_name
            }
            _ => {
                let is_option_type = is_option_type(&field.ty);
                if is_option_type {
                    declarations.extend(quote! {
                        let mut #field_name: #type_name = None;
                    });
                } else {
                    declarations.extend(quote! {
                        let mut #field_name: Option<#type_name>> = None;
                    });
                }
                field_name.clone()
            }
        };

        let Some(meta) = meta else {
            continue;
        };

        match_statements.extend(quote! {
            #field_name_str => {
                if #var_name.is_some() {
                    return Err(serde::de::Error::duplicate_field(#field_name_str));
                }
                #var_name = Some(access.next_value()?);
            }
        });

        match meta {
            Metadata::Style => {
                color_resolution.extend(quote! {
                    let mut #field_name = ratatui::style::Style::default();
                    if let Some(#var_name) = #var_name {
                        if let Some(fg) = #struct_name::__resolve_fg_color(&#var_name, &color_map) {
                            #field_name = #field_name.fg(fg);
                        }
                        if let Some(bg) = #struct_name::__resolve_bg_color(&#var_name, &color_map) {
                            #field_name = #field_name.bg(bg);
                        }
                    }
                });
            }
            Metadata::Styles(fields) => {
                let field_assignments = fields.iter().map(|field| {
                    quote! {
                        if let Some(color_str) = proxy.#field.fg {
                            if let Some(color) = #struct_name::__resolve_color_str(&color_str, &color_map) {
                                dialog.#field = dialog.#field.fg(color);
                            }
                        }

                        if let Some(color_str) = proxy.#field.bg {
                            if let Some(color) = #struct_name::__resolve_color_str(&color_str, &color_map) {
                                dialog.#field = dialog.#field.bg(color);
                            }
                        }
                    }
                });

                color_resolution.extend(quote! {
                let mut #field_name: #type_name = unsafe { std::mem::zeroed() };
                if let Some(proxy) = #var_name {
                        #(#field_assignments)*
                    }
                });
            }
        }
    }

    let fields = quote! {
        #(#field_names),*
    };

    let initialize_theme = quote! {
        Ok(#struct_name { #fields })
    };

    let deserialize_implementation = quote! {
        impl<'de> ratatheme_types::DeserializeTheme<'de> for #struct_name {
            fn deserialize_theme<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct ThemeVisitor;

                impl<'de> serde::de::Visitor<'de> for ThemeVisitor {
                    type Value = #struct_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a theme with 'colors' and 'base' sections")
                    }

                    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                    where
                        M: serde::de::MapAccess<'de>,
                    {
                        let mut color_map: Option<std::collections::HashMap<String, String>> = None;
                        #declarations

                        while let Some(key) = access.next_key::<String>()? {
                            match String::as_str(&key) {
                                "colors" => {
                                    if color_map.is_some() {
                                        return Err(serde::de::Error::duplicate_field("colors"));
                                    }
                                    color_map = Some(access.next_value()?);
                                }
                                #match_statements
                                _ => {
                                    let _ignored: serde::de::IgnoredAny = access.next_value()?;
                                }
                            }
                        }

                        let color_map = color_map.unwrap_or_default();

                        #color_resolution

                        #initialize_theme
                    }
                }


                deserializer.deserialize_map(ThemeVisitor)
            }
        }
    };

    let helper_functions = get_color_resolver_fns();

    let expanded = quote! {
        #deserialize_implementation

        impl #struct_name {
            #helper_functions
        }
    };

    TokenStream::from(expanded)
}

/// Helper method to handle a single `Field` of the target struct.
fn handle_field(field: &Field) -> Option<Metadata> {
    let attr = find_attribute(&field.attrs)?;

    let meta = find_metadata(attr)?;

    Some(meta)
}

/// Helper to find the `theme` attribute in a list of attributes.
fn find_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("theme"))
}

/// Helper to find the `Metadata` of an attribute.
fn find_metadata(attr: &Attribute) -> Option<Metadata> {
    let Meta::List(list) = &attr.meta else {
        panic!("expected metadata in the format 'theme(style)'");
    };

    let mut iter = list.tokens.clone().into_iter();
    let ident = iter.next()?;

    match ident.to_string().as_str() {
        "style" => Some(Metadata::Style),
        "styles" => {
            let Some(TokenTree::Group(group)) = iter.next() else {
                return None;
            };

            let mut fields = Vec::new();
            for field in group.stream() {
                if let TokenTree::Ident(ident) = field {
                    fields.push(ident);
                }
            }

            Some(Metadata::Styles(fields))
        }
        ident => panic!("unexpected metadata: {ident}, supported: style, colors or styles(..)"),
    }
}

/// Helper method to build the color resolver functions.
fn get_color_resolver_fns() -> TokenStream2 {
    quote! {
        fn __resolve_fg_color(
            style_map: &std::collections::HashMap<String, String>,
            color_map: &std::collections::HashMap<String, String>,
        ) -> Option<ratatui::style::Color> {
            let color_str = style_map.get("fg").or_else(|| style_map.get("foreground"))?;
            Self::__resolve_color_str(color_str, color_map)
        }

        fn __resolve_bg_color(
            style_map: &std::collections::HashMap<String, String>,
            color_map: &std::collections::HashMap<String, String>,
        ) -> Option<ratatui::style::Color> {
            let color_name = style_map.get("bg").or_else(|| style_map.get("background"))?;
            Self::__resolve_color_str(color_name, color_map)
        }

        fn __resolve_color_str(color_str: &str, color_map: &std::collections::HashMap<String, String>) -> Option<ratatui::style::Color> {
            use std::str::FromStr;
            if let Some(color) = color_map
                .get(color_str)
                .and_then(|color_value| ratatui::style::Color::from_str(color_value).ok())
            {
                return Some(color);
            }
            ratatui::style::Color::from_str(color_str).ok()
        }
    }
}

/// Represents all attribute metadatas that are supported.
enum Metadata {
    Style,
    Styles(Vec<Ident>),
}

/// A helper method that checks whether a fields type is an Option.
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                    return !arguments.args.is_empty();
                }
            }
        }
    }
    false
}

/// A helper method to extract the identifier of a field's type, if it exists
fn get_type_identifier(ty: &Type) -> Option<syn::Ident> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.first() {
            return Some(segment.ident.clone());
        }
    }
    None
}
