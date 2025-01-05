use ratatui::style::{Color, Style};
use serde::Deserialize;
use tui_theme_builder_internal::ThemeBuilder;

#[derive(Debug, Deserialize)]
pub struct MyColor {
    pub primary: Color,
}

impl Default for MyColor {
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
}

impl ThemeBuilder for Theme {
    type Context = MyColor;

    fn build(context: &Self::Context) -> Self {
        Self {
            base: Style::default().fg(ratatui::style::Color::from(context.primary.clone())),
        }
    }
}

impl From<MyColor> for Theme {
    fn from(value: MyColor) -> Self {
        Self {
            base: Style::default().fg(ratatui::style::Color::from(value.primary.clone())),
        }
    }
}

fn main() {
    let color = MyColor::default();
    println!("{:#?}", color.primary);
}
