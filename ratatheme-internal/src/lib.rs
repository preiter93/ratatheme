use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// A color wrapper for `ratatheme`.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Color {
    inner: csscolorparser::Color,
}

impl Color {
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: csscolorparser::Color::from_rgba8(r, g, b, 255),
        }
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self {
            inner: csscolorparser::Color::from_hsla(h, s, l, 1.0),
        }
    }

    /// Get the color as rgb.
    pub fn to_rgb(&self) -> [u8; 3] {
        let [r, g, b, _] = self.inner.to_rgba8();
        [r, g, b]
    }

    /// Get the color as hsl.
    pub fn to_hsl(&self) -> [f32; 3] {
        let [h, s, l, _] = self.inner.to_hsla();
        [h, s, l]
    }

    /// Get the color as hex string.
    pub fn to_hex_string(&self) -> String {
        self.inner.to_hex_string()
    }
}

impl FromStr for Color {
    type Err = csscolorparser::ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = csscolorparser::parse(s)?;
        Ok(Self { inner: color })
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        let [r, g, b] = value.to_rgb();
        Self::Rgb(r, g, b)
    }
}

pub trait ThemeBuilder {
    type Context;
    fn build(context: &Self::Context) -> Self;
}
