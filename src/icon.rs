// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// d5fa1e07c93be2a170b0c7648bcb02d4e9c30f3e1a76eb203690e82a880f9706
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("check", "\u{E06C}"),
    ("chevron_r", "\u{E06F}"),
    ("circle", "\u{E076}"),
    ("circle_alert", "\u{E077}"),
    ("circle_dashed", "\u{E4B0}"),
    ("ellipsis_v", "\u{E0B7}"),
    ("trash", "\u{E18E}"),
];

pub fn check<'a>() -> Text<'a> {
    icon("\u{E06C}")
}

pub fn chevron_r<'a>() -> Text<'a> {
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

pub fn ellipsis_v<'a>() -> Text<'a> {
    icon("\u{E0B7}")
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
