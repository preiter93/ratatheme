# ratatheme

`ratatheme` is a convenient theme parser for `ratatui` apps. It simplifies defining and deserializing themes from configuration 
files into a theme with `ratatui::Style`s.

## What does `ratatheme` try to solve?

Ratatui themes are often defined as a collection of `ratatui::style::Styles`. However, these styles do not provide a convenient way to define
a theme configuration. The main idea of ratatheme is that a mapping from color name to hex color can be specified alongside the theme config. 
The styles can then be defined using the color name and `ratatheme` takes care of looking the color name up in the color map.

Note that this is similar to css, where you can define global color variables and use them with var(--my-color).

## Central elements

- **`DeserializeTheme`**: A custom `Deserialize` proc macro.
- **`style` attribute**: Annotate fields with `#[theme(style)]` to specify that these fields will be deserialized and used to generate `ratatui::Style` objects. The `foreground` and `background` fields of the serialized data can hold color names which will be references from a map of colors. See below.
- **`Subtheme`**: For nested themes, the struct must derive the `Subtheme` proc macro.

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
    // Use the `style` attribute to indicate this field should be deserialized smartly.
    #[theme(style)]
    base: Style,

    // Use the `styles` attribute to define the styles of a subtheme.
    #[theme(styles(info, warn))]
    dialog: DialogTheme,
}

// Themes must implement `Subtheme`.
#[derive(Debug, Subtheme)]
struct DialogTheme {
    info: Style,
    warn: Style,
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
        warn.foreground = "my_red"
    "##;

        let deserializer = toml::Deserializer::new(toml_str);
        Theme::deserialize(deserializer).unwrap()
    }
}
```
