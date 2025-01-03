use ratatheme::{DeserializeTheme, Subtheme};
use ratatui::style::Style;

fn main() {
    let theme = Theme::default();
    println!("theme {theme:#?}");
}

// Theme must implement `DeserializeTheme`.
#[derive(Debug, DeserializeTheme)]
pub struct Theme {
    // Use the `style` attribute to indicate that this field is parsed with
    // the custom deserializer. It should only be applied to fields of type
    // `ratatui::style::Styles`.
    #[theme(style)]
    pub base: Style,

    // Use the `styles` attribute to define the styles of a subtheme.
    #[theme(styles(info))]
    pub dialog: DialogTheme,

    // Fields that are not annotated are parsed with `serde::Deserialize`.
    pub hide: Option<bool>,
}

// Subthemes must implement `Subtheme`.
#[derive(Debug, Subtheme)]
pub struct DialogTheme {
    // Use the `styles` attribute also on the subtheme's styles.
    #[theme(style)]
    pub info: Style,
}

impl Default for Theme {
    fn default() -> Self {
        let toml_str = r##"
        [colors]
        "my_red" = "#d32f2f"
        "my_blue" = "#1976d2"
        "my_green" = "#388e3c"

        [base]
        foreground = "my_red"
        background = "my_green"

        [dialog]
        info.foreground = "my_blue"
    "##;

        let deserializer = toml::Deserializer::new(toml_str);
        Self::deserialize_theme(deserializer).unwrap()
    }
}
