// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// e068ee1b0e6f90021cfe0e84dee4ef5d62a872323e0b85fe8a56a50071d18b1e
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("check", "\u{E06C}"),
    ("chevron_left", "\u{E06E}"),
    ("chevron_right", "\u{E06F}"),
    ("circle", "\u{E076}"),
    ("circle_alert", "\u{E077}"),
    ("circle_dashed", "\u{E4B0}"),
    ("ellipsis_vertical", "\u{E0B7}"),
    ("list_chevrons_up_down", "\u{E696}"),
    ("trash", "\u{E18E}"),
];

pub fn check<'a>() -> Text<'a> {
    icon("\u{E06C}")
}

pub fn chevron_left<'a>() -> Text<'a> {
    icon("\u{E06E}")
}

pub fn chevron_right<'a>() -> Text<'a> {
    icon("\u{E06F}")
}

pub fn circle<'a>() -> Text<'a> {
    icon("\u{E076}")
}

pub fn circle_alert<'a>() -> Text<'a> {
    icon("\u{E077}")
}

pub fn circle_dashed<'a>() -> Text<'a> {
    icon("\u{E4B0}")
}

pub fn ellipsis_vertical<'a>() -> Text<'a> {
    icon("\u{E0B7}")
}

pub fn list_chevrons_up_down<'a>() -> Text<'a> {
    icon("\u{E696}")
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
