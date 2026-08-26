// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 5271591ece5496cd0a679b0eef048228264743110fb8cedb4600b37160176ad5
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("edit", "\u{E1F9}"),
    ("save", "\u{E14D}"),
    ("search", "\u{E151}"),
    ("trash", "\u{E18E}"),
];

pub fn edit<'a>() -> Text<'a> {
    icon("\u{E1F9}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{E14D}")
}

pub fn search<'a>() -> Text<'a> {
    icon("\u{E151}")
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
