use ratatui::style::{Color, Modifier, Style};
use tui_theme_builder::ThemeBuilder;

#[test]
fn theme_with_style() {
    struct Colors {
        primary: Color,
    }
    #[derive(ThemeBuilder)]
    #[builder(context=Colors)]
    struct Theme {
        #[style(fg=primary,bg=primary)]
        style: Style,
    }

    let colors = Colors {
        primary: Color::Black,
    };
    let theme = Theme::build(&colors);
    assert_eq!(theme.style.fg, Some(Color::Black));
    assert_eq!(theme.style.bg, Some(Color::Black));
}

macro_rules! test_modifier {
    ($name:ident, $modifier_name:ident, $modifier:expr) => {
        #[test]
        fn $name() {
            struct Colors {}
            #[derive(ThemeBuilder)]
            #[builder(context=Colors)]
            struct Theme {
                #[style($modifier_name)]
                style: Style,
            }

            let colors = Colors {};
            let theme = Theme::build(&colors);

            assert_eq!(theme.style.add_modifier, $modifier);
        }
    };
}
test_modifier!(theme_with_bold, bold, Modifier::BOLD);
test_modifier!(theme_with_underlined, underlined, Modifier::UNDERLINED);
test_modifier!(theme_with_hidden, hidden, Modifier::HIDDEN);
test_modifier!(theme_with_rapid_blink, rapid_blink, Modifier::RAPID_BLINK);
test_modifier!(theme_with_slow_blink, slow_blink, Modifier::SLOW_BLINK);
test_modifier!(theme_with_dim, dim, Modifier::DIM);
test_modifier!(theme_with_italic, italic, Modifier::ITALIC);
test_modifier!(theme_with_reversed, reversed, Modifier::REVERSED);
test_modifier!(theme_with_crossed_out, crossed_out, Modifier::CROSSED_OUT);

#[test]
fn theme_with_annotated_field() {
    struct Context {
        hide: bool,
    }
    #[derive(ThemeBuilder)]
    #[builder(context=Context)]
    struct Theme {
        #[builder(value=hide)]
        hide: bool,
    }

    let context = Context { hide: true };
    let theme = Theme::build(&context);
    assert_eq!(theme.hide, true);
}

#[test]
fn theme_with_default_field() {
    struct Context {}
    #[derive(ThemeBuilder)]
    #[builder(context=Context)]
    struct Theme {
        #[builder(value=default)]
        hide: bool,
    }

    let context = Context {};
    let theme = Theme::build(&context);
    assert_eq!(theme.hide, false);
}

#[test]
fn theme_with_colors_from_context() {
    struct Colors {
        primary: Color,
    }
    struct Context {
        colors: Colors,
    }
    #[derive(ThemeBuilder)]
    #[builder(context=Context)]
    struct Theme {
        #[style(fg=colors.primary,bg=colors.primary)]
        style: Style,
    }

    let context = Context {
        colors: Colors {
            primary: Color::Black,
        },
    };
    let theme = Theme::build(&context);
    assert_eq!(theme.style.fg, Some(Color::Black));
    assert_eq!(theme.style.bg, Some(Color::Black));
}

#[test]
fn theme_with_subtheme() {
    struct Colors {
        primary: Color,
    }
    #[derive(ThemeBuilder)]
    #[builder(context=Colors)]
    struct Theme {
        sub_theme: Subtheme,
    }

    #[derive(ThemeBuilder)]
    #[builder(context=Colors)]
    struct Subtheme {
        #[style(foreground=primary)]
        style: Style,
    }

    let colors = Colors {
        primary: Color::Black,
    };
    let theme = Theme::build(&colors);
    assert_eq!(theme.sub_theme.style.fg, Some(Color::Black));
    assert_eq!(theme.sub_theme.style.bg, None);
}
