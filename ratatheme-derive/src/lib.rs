use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod deserialize;
mod subtheme;

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
    deserialize::impl_deserialize_theme(&input)
}

#[proc_macro_derive(Subtheme)]
pub fn subtheme_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    subtheme::expand_subtheme(&input)
}
