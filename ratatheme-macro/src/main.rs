#![allow(dead_code, unused_variables)]
use ratatheme_macro::Theme;
use ratatheme_types::{Colors, Style};

fn main() {
    let theme = Theme::default();

    let base = theme.base();
    println!("base {base:?}\n");

    let colors = theme.colors.clone();
    println!("colors {colors:?}\n");

    println!("dialog_info {:?}\n", theme.dialog_info());

    println!("dialog_warn {:?}\n", theme.dialog_warn());
}

#[derive(Theme)]
struct Theme {
    #[theme(colors)]
    colors: Colors,

    #[theme(style)]
    base: Style,

    #[theme(styles(info, warn))]
    dialog: Dialogtheme,
}

#[derive(Default)]
struct Dialogtheme {
    info: Style,
    warn: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: Colors::new(vec![
                ("funky-red", "#ff0000"),
                ("funky-blue", "#0000ff"),
                ("funky-green", "#00ff00"),
            ]),
            base: Style::default().foreground("funky-red"),
            dialog: Dialogtheme {
                info: Style::default().foreground("funky-blue"),
                warn: Style::default().foreground("funky-red"),
            },
        }
    }
}
