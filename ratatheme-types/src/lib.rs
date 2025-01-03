use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize a theme for Ratatui applications from a human-friendly format.
///
/// The [`DeserializeTheme`] trait should not be implemented manually.
/// It is derived within the `ratatheme-derive` crate.
pub trait DeserializeTheme<'de>: Sized {
    /// Deserialize a theme from the given Serde deserializer.
    ///
    /// # Errors
    /// Errors if deserialization fails.
    fn deserialize_theme<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

/// A simple proxy for `ratatui::Style`, designed to simplify deserialization.
///
/// This struct serves as a proxy for handling the style information like
/// foreground and background colors in a way thats easier to deserialize.
///
/// The `ratatheme` crate takes care of converting this `Style` struct into
/// the appropriate `ratatui::Style` format. It is not intended that this
/// `Style` is being used directly.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Style {
    /// The foreground color name.
    #[cfg_attr(feature = "serde", serde(alias = "foreground"))]
    pub fg: Option<String>,

    /// The background color name.
    #[cfg_attr(feature = "serde", serde(alias = "background"))]
    pub bg: Option<String>,
}
