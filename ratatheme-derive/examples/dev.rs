use ratatheme_internal::Color;
use ratatui::style::Style;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MyColor {
    pub primary: Color,
}

impl Default for MyColor {
    fn default() -> Self {
        let toml_str = r##"
            "primary" = "rgb(1,0,0)"
        "##;
        let deserializer = toml::Deserializer::new(&toml_str);
        Self::deserialize(deserializer).unwrap()
    }
}

pub struct Theme {
    pub base: Style,
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
    println!("{:#?}", color.primary.to_rgb());
}
