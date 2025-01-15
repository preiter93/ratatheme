use ratatui::style::{Color, Style, Stylize};
use serde::Deserialize;
use tui_theme_builder::ThemeBuilder;

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub primary: Color,
}

#[derive(Debug, ThemeBuilder, PartialEq, Eq)]
#[builder(context=Colors)]
pub struct Theme {
    /// Annotate styles with 'fg=color', 'bg=color' or add modifiers,
    /// e.g. 'bold' or 'underlined'.
    #[style(fg=primary, bg=primary, bold, underlined)]
    pub base: Style,

    /// Annotate sub-styles with 'builder(child)'
    #[builder(child)]
    pub sub_theme: Subtheme,
}

#[derive(Debug, ThemeBuilder, PartialEq, Eq)]
#[builder(context=Colors)]
pub struct Subtheme {
    #[style(fg=primary, bg=primary)]
    pub base: Style,
}

impl Default for Colors {
    fn default() -> Self {
        let s = r##"
        "primary" = "#000000"
        "##;
        toml::from_str(s).unwrap()
    }
}

fn main() {
    let colors = Colors::default();
    let theme = Theme::build(&colors);
    println!("{theme:#?}");

    assert_eq!(
        theme,
        Theme {
            base: Style::default()
                .fg(ratatui::style::Color::Rgb(0, 0, 0))
                .bg(ratatui::style::Color::Rgb(0, 0, 0))
                .bold()
                .underlined(),
            sub_theme: Subtheme {
                base: Style::default()
                    .fg(ratatui::style::Color::Rgb(0, 0, 0))
                    .bg(ratatui::style::Color::Rgb(0, 0, 0)),
            }
        }
    );
}
