use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Fields;
use syn::Ident;
use syn::{Data, DeriveInput};

use crate::helpers::find_attribute_metadata;
use crate::helpers::find_theme_attribute;
use crate::helpers::get_type_token_stream;
use crate::helpers::Metadata;

pub(super) fn subtheme_proxy_name(struct_name: &Ident) -> Ident {
    format_ident!("{}Proxy", struct_name)
}

pub(super) fn expand_subtheme(input: &DeriveInput) -> TokenStream {
    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let struct_name = input.ident.clone();

    let Fields::Named(named_fields) = &data.fields else {
        return TokenStream::new();
    };

    let proxy_name = subtheme_proxy_name(&struct_name);

    let mut field_declaration = Vec::new();
    let mut from_assignment = Vec::new();

    for field in &named_fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let meta = find_theme_attribute(&field.attrs).and_then(find_attribute_metadata);

        let (declaration, assignment) = match meta {
            Some(Metadata::Style) => (
                quote! { #field_name: ratatheme_types::Style },
                quote! { #field_name: ratatui::style::Style::default() },
            ),
            None => {
                let type_name = get_type_token_stream(&field.ty);
                (
                    quote! { #field_name: #type_name },
                    quote! { #field_name: value.#field_name },
                )
            }
            Some(_) => panic!("Only the 'style' attribute is supported for subtheme"),
        };

        field_declaration.push(declaration);
        from_assignment.push(assignment);
    }

    let expanded = quote! {
        #[derive(Debug, ::serde::Deserialize)]
        struct #proxy_name {
            #(#field_declaration),*
        }

        impl From<#proxy_name> for #struct_name {
            fn from(value: #proxy_name) -> Self {
                Self {
                    #(#from_assignment),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// impl From<DialogThemeProxy> for DialogTheme {
//     fn from(value: DialogThemeProxy) -> Self {
//         DialogTheme {
//             info: ratatui::style::Style::default(),
//             hide: value.hide,
//         }
//     }
// }
