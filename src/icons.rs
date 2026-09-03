// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 150948d444cfbdb343a02ecad6f8b5914d89fcbb55674d0bbb7bfd22a9c269d5
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("ellipsis_vertical", "\u{E0B7}"),
    ("list_chevrons_up_down", "\u{E696}"),
    ("signal_high", "\u{E260}"),
    ("signal_low", "\u{E261}"),
    ("signal_medium", "\u{E262}"),
    ("trash", "\u{E18E}"),
];

pub fn ellipsis_vertical<'a>() -> Text<'a> {
    icon("\u{E0B7}")
}

pub fn list_chevrons_up_down<'a>() -> Text<'a> {
    icon("\u{E696}")
}

pub fn signal_high<'a>() -> Text<'a> {
    icon("\u{E260}")
}

pub fn signal_low<'a>() -> Text<'a> {
    icon("\u{E261}")
}

pub fn signal_medium<'a>() -> Text<'a> {
    icon("\u{E262}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E18E}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Msg::Pick(name.to_string()))
/// }
/// ```
pub fn render(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("lucide"))
}

fn icon(codepoint: &str) -> Text<'_> {
    render(codepoint)
}
