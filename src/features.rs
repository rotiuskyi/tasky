pub mod app;
pub mod tasks;

pub const TITLE_SIZE_MD: u32 = 28;

/// Medium text_editor height calculated for default font size
fn editor_height_md() -> f32 {
    let font_size = 16.0;
    let lines_to_show = 4.0;
    let vertical_padding = 24.0;

    (font_size * lines_to_show) + vertical_padding
}
