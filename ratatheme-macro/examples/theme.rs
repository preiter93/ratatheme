use ratatheme_macro::Theme;
use ratatheme_types::{Colors, Style};
use serde::{Deserialize, Serialize};

#[derive(Theme, Debug, Serialize, Deserialize)]
struct Theme {
    #[theme(colors)]
    colors: Colors,

    #[theme(style)]
    base: Style,

    #[theme(styles(info, warn))]
    dialog: Dialog,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Dialog {
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

        [dialog]
        info.foreground = "blue"
        warn.foreground = "red"
    "##;
        toml::from_str(&toml_str).unwrap()
    }
}

fn main() {
    let theme = Theme::default();
    println!("{theme:#?}");

    println!("base-style: {:#?}\n", theme.base());

    println!("dialog-info-style: {:#?}\n", theme.dialog_info());

    println!("dialog-warn-style: {:#?}\n", theme.dialog_warn());
}
