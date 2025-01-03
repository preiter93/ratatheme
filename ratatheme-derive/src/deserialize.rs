use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident};

use crate::helpers::{
    find_attribute_metadata, find_theme_attribute, get_type_identifier, get_type_token_stream,
    is_option_type, Metadata,
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

    let mut variable_definition = quote! {};
    let mut deserialization = quote! {};
    let mut match_statement = quote! {};

    for field in &fields.named {
        let meta = find_theme_attribute(&field.attrs).and_then(find_attribute_metadata);

        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        field_names.push(field_name.clone());

        let type_name = get_type_token_stream(&field.ty).unwrap();

        let var_name = match meta {
            Some(Metadata::Style) => {
                let map_name = format_ident!("{field_name}_map");
                variable_definition.extend(quote! {
                    let mut #map_name: Option<std::collections::HashMap<String, String>> = None;
                });
                map_name
            }
            Some(Metadata::Styles(_)) => {
                let type_ident = get_type_identifier(&field.ty).unwrap();
                let proxy_struct_name = subtheme_proxy_name(&type_ident);
                let proxy_var_name = format_ident!("{field_name}_proxy");

                variable_definition.extend(quote! {
                    let mut #proxy_var_name: Option<#proxy_struct_name> = None;
                });

                proxy_var_name
            }
            _ => {
                let is_option_type = is_option_type(&field.ty);
                if is_option_type {
                    variable_definition.extend(quote! {
                        let mut #field_name: #type_name = None;
                    });
                } else {
                    variable_definition.extend(quote! {
                        let mut #field_name: Option<#type_name> = None;
                    });
                }
                field_name.clone()
            }
        };

        match_statement.extend(quote! {
            #field_name_str => {
                if #var_name.is_some() {
                    return Err(serde::de::Error::duplicate_field(#field_name_str));
                }
                #var_name = Some(access.next_value()?);
            }
        });

        match meta {
            Some(Metadata::Style) => {
                deserialization.extend(quote! {
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
            Some(Metadata::Styles(fields)) => {
                let type_name = get_type_token_stream(&field.ty);

                let bind_variables = fields.iter().map(|field| {
                    let fg_name = format_ident!("fg_{field}");
                    let bg_name = format_ident!("bg_{field}");
                    quote! {
                        let #fg_name = proxy.#field.fg.take();
                        let #bg_name = proxy.#field.bg.take();
                    }
                });

                let variable_assignment = fields.iter().map(|field| {
                    let fg_name = format_ident!("fg_{field}");
                    let bg_name = format_ident!("bg_{field}");
                    quote! {
                        if let Some(color_str) = #fg_name {
                            if let Some(color) = #struct_name::__resolve_color_str(&color_str, &color_map) {
                                original.#field = original.#field.fg(color);
                            }
                        }
                        if let Some(color_str) = #bg_name {
                            if let Some(color) = #struct_name::__resolve_color_str(&color_str, &color_map) {
                                original.#field = original.#field.bg(color);
                            }
                        }
                    }
                });

                deserialization.extend(quote! {
                    let #field_name = #var_name.take().map_or_else(
                        || unsafe { std::mem::zeroed() },
                        |mut proxy| {
                            #(#bind_variables)*
                            let mut original: #type_name = proxy.into();
                            #(#variable_assignment)*
                            original
                        },
                    );
                });
            }
            _ => {
                let is_option_type = is_option_type(&field.ty);
                if !is_option_type {
                    deserialization.extend(quote! {
                        let #var_name = #var_name.unwrap();
                    });
                }
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
                        #variable_definition

                        while let Some(key) = access.next_key::<String>()? {
                            match String::as_str(&key) {
                                "colors" => {
                                    if color_map.is_some() {
                                        return Err(serde::de::Error::duplicate_field("colors"));
                                    }
                                    color_map = Some(access.next_value()?);
                                }
                                #match_statement
                                _ => {
                                    let _ignored: serde::de::IgnoredAny = access.next_value()?;
                                }
                            }
                        }

                        let color_map = color_map.unwrap_or_default();

                        #deserialization

                        #initialize_theme
                    }
                }


                deserializer.deserialize_map(ThemeVisitor)
            }
        }
    };

    let helper_functions = quote! {
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
    };

    let expanded = quote! {
        #deserialize_implementation

        impl #struct_name {
            #helper_functions
        }
    };

    TokenStream::from(expanded)
}
