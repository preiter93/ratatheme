<div align="center">

# tui-theme-builder
 
[![Deps.rs Badge]][Deps.rs]

A theme builder macro for [ratatui](https://github.com/ratatui/ratatui) apps.

</div>


```rust
use ratatui::style::{Color, Style};
use serde::Deserialize;
use tui_theme_builder::ThemeBuilder;

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub primary: Color,
}

#[derive(ThemeBuilder)]
#[builder(context=Colors)]
pub struct Theme {
    /// Annotate styles with 'fg=color', 'bg=color' or add modifiers,
    /// e.g. 'bold' or 'underlined'.
    #[style(fg=primary, bg=primary, bold, underlined)]
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
}
```

## Apps using `tui-theme-builder`

- [Mantui](https://github.com/preiter93/mantui/blob/main/src/ui/theme.rs)

[Deps.rs Badge]: https://deps.rs/repo/github/preiter93/tui-theme-builder/status.svg?path=tui-theme-builder&style=flat-square
[Deps.rs]: https://deps.rs/repo/github/preiter93/tui-theme-builder?path=tui-theme-builder
