use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Fields;
use syn::Ident;
use syn::{Data, DeriveInput};

pub(super) fn subtheme_proxy_name(struct_name: &Ident) -> Ident {
    format_ident!("{}Proxy", struct_name)
}

pub(super) fn expand_subtheme(input: &DeriveInput) -> TokenStream {
    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let struct_name = input.ident.clone();

    let Fields::Named(fields) = &data.fields else {
        return TokenStream::new();
    };

    let proxy_name = subtheme_proxy_name(&struct_name);

    let fields = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            #field_name: ratatheme_types::Style
        }
    });

    let expanded = quote! {
        #[derive(::serde::Deserialize)]
        struct #proxy_name {
            #(#fields),*
        }
    };

    TokenStream::from(expanded)
}
