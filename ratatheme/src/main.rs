#![allow(unused)]
use ratatheme::{DeserializeTheme, Subtheme};
use ratatui::style::Style;

fn main() {
    let theme = Theme::default();
    println!("theme {theme:#?}");
}

#[derive(Debug, DeserializeTheme)]
struct Theme {
    #[theme(style)]
    base: Style,
    #[theme(styles(info, warn))]
    dialog: DialogTheme,
}

#[derive(Debug, Subtheme)]
struct DialogTheme {
    info: Style,
    warn: Style,
}

impl Default for Theme {
    fn default() -> Self {
        let toml_str = r##"
        [colors]
        "red" = "#d32f2f"
        "blue" = "#1976d2"
        "green" = "#388e3c"

        [base]
        foreground = "red"
        background = "green"

        [dialog]
        info.foreground = "blue"
        warn.foreground = "red"
    "##;
        let deserializer = toml::Deserializer::new(toml_str);
        Theme::deserialize(deserializer).unwrap()
    }
}
