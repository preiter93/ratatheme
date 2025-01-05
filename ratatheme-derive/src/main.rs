use ratatheme_derive::ThemeBuilder;
use ratatheme_internal::Color;
use ratatheme_internal::ThemeBuilder;
use ratatui::style::{Style, Stylize};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub primary: Color,
    pub footer: Footer,
}
#[derive(Debug, Default, Deserialize)]
pub struct Footer {
    pub hide: bool,
}

#[derive(Debug, ThemeBuilder, PartialEq, Eq)]
#[builder(context=Config)]
pub struct Theme {
    /// Annotate styles with 'fg', 'bg' or any modifier, e.g. 'bold'.
    #[style(fg=primary, bg=primary, bold, underlined)]
    pub base: Style,

    /// Note: fields can also be annoted with `builder` to values from context.
    #[builder(value=footer.hide)]
    pub hide: bool,

    /// Note: untagged fields must implement default.
    pub tags: usize,
}

fn main() {
    let color = Config {
        primary: Color::from_rgb(0, 0, 0),
        footer: Footer { hide: true },
    };
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
            hide: true,
            tags: 0,
        }
    );
}
