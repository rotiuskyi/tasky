// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 7f2b5cc376faa51acdd922bcaf4690a558c5a9eeb0b3f92149951f6e7ff14e69
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[("ellipsis_vertical", "\u{E0B7}"), ("trash", "\u{E18E}")];

pub fn ellipsis_vertical<'a>() -> Text<'a> {
    icon("\u{E0B7}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E18E}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Message::Pick(name.to_string()))
/// }
/// ```
pub fn render(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("lucide"))
}

fn icon(codepoint: &str) -> Text<'_> {
    render(codepoint)
}
