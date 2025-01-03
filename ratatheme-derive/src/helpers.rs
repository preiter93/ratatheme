use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::{Attribute, Ident, Meta, PathArguments, Type};

/// A helper method that checks whether a fields type is an Option.
pub(super) fn is_option_type(ty: &Type) -> bool {
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

/// A helper method to extract the identifier of a field's type, if it exists.
pub(super) fn get_type_identifier(ty: &syn::Type) -> Option<Ident> {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        return path.get_ident().cloned();
    }
    None
}

/// A helper method to extract a token stream of a field's type, if it exists
pub(super) fn get_type_token_stream(ty: &syn::Type) -> Option<TokenStream2> {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        let stream = path.into_token_stream();
        return Some(stream);
    }
    None
}

/// A helper method to find the `theme` attribute in a list of attributes.
pub(super) fn find_theme_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("theme"))
}

/// Helper to find the `Metadata` of a `themes` attribute.
pub(super) fn find_attribute_metadata(attr: &Attribute) -> Option<Metadata> {
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

/// Represents all attribute metadatas that are supported.
pub(super) enum Metadata {
    Style,
    Styles(Vec<Ident>),
}
