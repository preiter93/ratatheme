# ratatheme

`ratatheme` is a convenient theme parser for [ratatui](https://github.com/ratatui/ratatui) applications. It simplifies deserialization of themes from configuration files into a theme that holds `ratatui::Style`'s.

## What does `ratatheme` attempt to solve?

Ratatui themes are often defined as a collection of `ratatui::style::Style`'s. And themes may be defined in a serialized format such as a toml configuration file. The current deserialization process for converting serialized data, such as a TOML string, into `ratatui::style::Style` is somewhat rigid. While `ratatui::style::Style` allows colors to be parsed from a string, the built-in parser only supports a limited set of color formats. It accepts for example colors in hex codes or a predefined set of standard color names like red, blue, etc.

But what if you wish to define a unique color, such as `my_color`, and reference it in your configuration?. Just similar to CSS where you would define a global color variable and use it with var(--my-color). Ideally, the deserialization process should support such custom color definitions, allowing users to define their own colors.

With “Ratatheme” you can define a color map from color name to hex code next to your configuration. The foreground and background color of the `ratatui::style::Style` can then be defined via the color name in your configuration file. `Ratatheme` takes care of resolving the color from the color map at deserialization time.

## Central elements

- **`DeserializeTheme`**: A custom `Deserialize` proc macro that implements `deserialize_theme`.
- Attribute `style`**: Fields annotated with `#[theme(style)]` implement a special style deserialization.
- **`Subtheme`**: A derive macro for subthemes.

Under the hood `ratatheme` provides a proc macro `DeserializeTheme` that provides a `deserialize_theme` method. Notice that this is similar to serdes `Deserialize` macro. A theme derives `DeserializeTheme` and each style field should be annotated with `#[theme(style)]`. Fields that are not annotated are simply deserialized using standard serde. Themes can also have one (and not more) layer of depth, i.e. subthemes. The subtheme field inside the Theme must be annotated with `styles(field_a, field_b)`. The subtheme struct itself must derive `Subtheme` and each style field must also be annotated with `#[theme(style)]`. Sounds complicated? Let's jump into an example.

## Usage

```rust
use ratatheme::{DeserializeTheme, Subtheme};
use ratatui::style::Style;

fn main() {
    let theme = Theme::default();
    println!("theme {theme:#?}");
}

// Theme must implement `DeserializeTheme`.
#[derive(Debug, DeserializeTheme)]
struct Theme {
    // Use the `style` attribute to indicate that this field is parsed with
    // the custom deserializer. It should only be applied to fields of type
    // `ratatui::style::Styles`.
    #[theme(style)]
    base: Style,

    // Use the `styles` attribute to define the styles of a subtheme.
    #[theme(styles(info))]
    dialog: DialogTheme,
    
    // Fields that are not annotated are parsed with `serde::Deserialize`.
    hide: Option<bool>,
}

// Subthemes must implement `Subtheme`.
#[derive(Debug, Subtheme)]
struct DialogTheme {
    // Use the `styles` attribute also on the subtheme's styles.
    #[theme(style)]
    info: Style,
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
```

In Cargo.toml you must specify `ratatheme` as well as `serde`.
```
ratatheme = { package = "ratatheme", git = "https://github.com/preiter93/ratatheme.git", branch = "main" }
serde = { version = "1.0" }
```
