use ratatui::style::{Color, Style};
use serde::Deserialize;
use tui_theme_builder_internal::ThemeBuilder;

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub primary: Color,
}

impl Default for Colors {
    fn default() -> Self {
        let toml_str = r##"
            "primary" = "#000000"
        "##;
        let deserializer = toml::Deserializer::new(&toml_str);
        Self::deserialize(deserializer).unwrap()
    }
}

pub struct Theme {
    pub base: Style,
    pub sub_theme: Subtheme,
}

pub struct Subtheme {
    pub base: Style,
}

impl ThemeBuilder for Theme {
    type Context = Colors;

    fn build(context: &Self::Context) -> Self {
        Self {
            base: Style::default().fg(ratatui::style::Color::from(context.primary.clone())),
            sub_theme: Subtheme::build(context),
        }
    }
}

impl ThemeBuilder for Subtheme {
    type Context = Colors;

    fn build(context: &Self::Context) -> Self {
        Self {
            base: Style::default().fg(ratatui::style::Color::from(context.primary.clone())),
        }
    }
}

fn main() {
    let color = Colors::default();
    println!("{:#?}", color.primary);
}
