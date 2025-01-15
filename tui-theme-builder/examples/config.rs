use ratatui::style::{Color, Style, Stylize};
use serde::Deserialize;
use tui_theme_builder::ThemeBuilder;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub colors: Colors,
    pub footer: Footer,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub primary: Color,
}

impl Default for Config {
    fn default() -> Self {
        let s = r##"
        [colors]
        "primary" = "#000000"

        [footer]
        hide = true
        "##;
        toml::from_str(s).unwrap()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Footer {
    pub hide: bool,
}

#[derive(Debug, ThemeBuilder, PartialEq, Eq)]
#[builder(context=Config)]
pub struct Theme {
    /// Annotate styles with 'fg=color', 'bg=color' or add modifiers,
    /// e.g. 'bold', 'underlined' etc.
    #[style(fg=colors.primary, bg=colors.primary, bold, underlined)]
    pub base: Style,

    /// Fields can also be annoted with `builder(value=x)` to
    /// assign values from the context. Note: in this case
    /// the type `Footer` must implement `Clone`.
    #[builder(value=footer)]
    pub footer: Footer,

    /// Note: fields tagged with `value=default` are initialized with
    /// Default::default().
    #[builder(value=default)]
    pub tags: usize,
}

fn main() {
    let color = Config::default();
    let theme = Theme::build(&color);
    println!("{theme:#?}");

    assert_eq!(
        theme,
        Theme {
            base: Style::default()
                .fg(ratatui::style::Color::Rgb(0, 0, 0))
                .bg(ratatui::style::Color::Rgb(0, 0, 0))
                .bold()
                .underlined(),
            footer: Footer { hide: true },
            tags: 0,
        }
    );
}
