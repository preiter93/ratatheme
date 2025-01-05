/// A trait for building a theme from a given context.
///
/// This trait is typically derived using `ratatheme::ThemeBuilder` from the `ratatheme-derive` crate.
pub trait ThemeBuilder {
    /// The type of context used to build the theme.
    type Context;

    /// Builds and returns an instance of the implementing type using the provided context.
    fn build(context: &Self::Context) -> Self;
}
