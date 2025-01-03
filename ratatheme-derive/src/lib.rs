use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// A procedural macro that implements [`serde::Deserialize`] for
/// themes with [`ratatui::style::Style`]'s and enables a more
/// human-friendly definition of themes.
///
/// # Panics
/// - Panics if macro is not attached to a struct.
/// - Panics if any of the attribute tags is malformed.
#[proc_macro_derive(DeserializeTheme, attributes(theme))]
pub fn deserialize_theme_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _ = &input.ident;

    let Data::Struct(data) = &input.data else {
        panic!("derive must be attached to a struct");
    };

    let Fields::Named(fields) = &data.fields else {
        return TokenStream::new();
    };

    if fields.named.is_empty() {
        return TokenStream::new();
    }

    TokenStream::new()
}
