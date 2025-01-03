use ratatheme_types::DeserializeTheme;
use ratatui::style::{Color, Style};
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{collections::HashMap, fmt, str::FromStr};

fn main() {
    let theme = Theme::default();
    println!("theme {theme:#?}");
}

#[derive(Debug)]
pub struct Theme {
    // #[theme(style)]
    pub base: Style,

    // #[theme(styles(info, warn))]
    pub dialog: DialogTheme,

    pub hide: bool,
}

#[derive(Debug, Default)]
pub struct DialogTheme {
    pub info: Style,
    pub hide: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DialogThemeProxy {
    info: ratatheme_types::Style,
    hide: Option<bool>,
}

impl From<DialogThemeProxy> for DialogTheme {
    fn from(value: DialogThemeProxy) -> Self {
        DialogTheme {
            info: ratatui::style::Style::default(),
            hide: value.hide,
        }
    }
}

impl From<DialogThemeProxy> for DialogTheme {
    fn from(value: DialogThemeProxy) -> Self {
        Self {
            info: ratatui::style::Style::default(),
            hide: value.hide,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        let toml_str = r##"
        hide = true

        [colors]
        "red" = "#d32f2f"
        "blue" = "#1976d2"
        "green" = "#388e3c"

        [base]
        foreground = "red"
        background = "green"

        [dialog]
        info.foreground = "blue"
        hide = true
    "##;
        let deserializer = toml::Deserializer::new(&toml_str);
        Theme::deserialize_theme(deserializer).unwrap()
    }
}

impl<'de> DeserializeTheme<'de> for Theme {
    fn deserialize_theme<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeVisitor;

        impl<'de> Visitor<'de> for ThemeVisitor {
            type Value = Theme;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a theme with 'colors' and 'base' sections")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut colors_map: Option<HashMap<String, String>> = None;
                let mut base_map: Option<HashMap<String, String>> = None;
                let mut dialog_proxy: Option<DialogThemeProxy> = None;

                let mut hide: Option<bool> = None;

                while let Some(key) = access.next_key::<String>()? {
                    match String::as_str(&key) {
                        "colors" => {
                            if colors_map.is_some() {
                                return Err(de::Error::duplicate_field("colors"));
                            }
                            colors_map = Some(access.next_value()?);
                        }
                        "base" => {
                            if base_map.is_some() {
                                return Err(de::Error::duplicate_field("base"));
                            }
                            base_map = Some(access.next_value()?);
                        }
                        "dialog" => {
                            if dialog_proxy.is_some() {
                                return Err(de::Error::duplicate_field("dialog"));
                            }
                            dialog_proxy = Some(access.next_value()?);
                        }
                        "hide" => {
                            if hide.is_some() {
                                return Err(de::Error::duplicate_field("hide"));
                            }
                            hide = Some(access.next_value()?);
                        }
                        _ => {
                            let _ignored: de::IgnoredAny = access.next_value()?;
                        }
                    }
                }

                let colors_map = colors_map.unwrap_or_default();

                let mut base = Style::default();
                if let Some(base_map) = base_map {
                    if let Some(fg) = resolve_fg_color(&base_map, &colors_map) {
                        base = base.fg(fg);
                    }
                    if let Some(bg) = resolve_bg_color(&base_map, &colors_map) {
                        base = base.bg(bg);
                    }
                }

                let dialog = dialog_proxy.take().map_or_else(
                    || unsafe { std::mem::zeroed() },
                    |mut proxy| {
                        let fg = proxy.info.fg.take();
                        let bg = proxy.info.bg.take();
                        let mut dialog: DialogTheme = proxy.into();

                        if let Some(color_str) = fg {
                            if let Some(color) = resolve_color_str(&color_str, &colors_map) {
                                dialog.info = dialog.info.fg(color);
                            }
                        }

                        if let Some(color_str) = bg {
                            if let Some(color) = resolve_color_str(&color_str, &colors_map) {
                                dialog.info = dialog.info.bg(color);
                            }
                        }

                        dialog
                    },
                );

                let hide = hide.unwrap();

                Ok(Theme { base, dialog, hide })
            }
        }

        deserializer.deserialize_map(ThemeVisitor)
    }
}

fn resolve_fg_color(
    base_map: &HashMap<String, String>,
    colors_map: &HashMap<String, String>,
) -> Option<Color> {
    let color_str = base_map.get("fg").or_else(|| base_map.get("foreground"))?;
    resolve_color_str(color_str, colors_map)
}

fn resolve_bg_color(
    base_map: &HashMap<String, String>,
    colors_map: &HashMap<String, String>,
) -> Option<Color> {
    let color_name = base_map.get("bg").or_else(|| base_map.get("background"))?;
    resolve_color_str(color_name, colors_map)
}

fn resolve_color_str(color_str: &str, colors_map: &HashMap<String, String>) -> Option<Color> {
    if let Some(color) = colors_map
        .get(color_str)
        .and_then(|color_value| Color::from_str(color_value).ok())
    {
        return Some(color);
    }
    Color::from_str(color_str).ok()
}
