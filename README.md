# ratatheme

`ratatheme` is a convenient theme parser for `ratatui` apps. It simplifies defining and deserializing themes from configuration 
files into a theme with `ratatui::Style`s.

## What does `ratatheme` try to solve?

Ratatui themes are often defined as a collection of `ratatui::style::Styles`. and themes may often be defined in a serialized format such as a toml configuration file. But ratatui Styles have the issue that the color only deserializes from either the hex xode or a limited number of color name keywords. But what if you want to define your own color `my_color` and reference it in your configuration? With `ratatheme` you can define a color map alongside your configuration.
The styles can then be defined using the color name and `ratatheme` takes care of resolving the color from the colormap.

Note that this is similar to css, where you can define global color variables and use them with var(--my-color).

## Central elements

Under the hood `ratatheme` provides a proc macro `DeserializeTheme` that provides a `deserialize_theme` method, similar to serdes `Deserialize` macro. Your themes have to derive `DeserializeTheme` and each style field should be annotated with `#[theme(style)]`. Fields that are not annotated are simply deserialized using serde but are not handled specially. Themes can also have a single layer of depth by annotaing with `styles` and deriong `Subtheme` on the type of the annotated field.

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
