use ratatheme_derive::ThemeBuilder;
use ratatheme_internal::Color;
use ratatui::style::{Style, Stylize};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Colors {
    pub primary: Color,
}

#[derive(Debug, ThemeBuilder, PartialEq, Eq)]
#[builder(colors = Colors)]
pub struct Theme {
    /// Annotate styles with 'fg', 'bg' or any modifier, e.g. 'bold'.
    #[style(fg(primary), bg(primary), bold, underlined)]
    pub base: Style,

    /// Note: other field must implement default.
    pub hide: bool,
}

fn main() {
    let color = Colors::default();
    let theme = Theme::build(color);
    println!("{theme:#?}");

    assert_eq!(
        theme,
        Theme {
            base: Style::default()
                .fg(ratatui::style::Color::Rgb(0, 0, 0))
                .bg(ratatui::style::Color::Rgb(0, 0, 0))
                .bold()
                .underlined(),
            hide: false,
        }
    );
}
