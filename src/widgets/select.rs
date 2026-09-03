//! A `pick_list` dressed to match the menus of the app.
use std::borrow::Borrow;

use iced::widget::overlay::menu;
use iced::widget::{PickList, pick_list};
use iced::{Color, Shadow, Theme, Vector, border};

use crate::widgets::CONTROL_RADIUS;

/// Creates a [`PickList`] with the default styles, ready to be overridden.
pub fn select<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
{
    pick_list(options, selected, on_select)
        .style(default)
        .menu_style(default_menu)
}

/// The field, matching the `button::subtle` menu trigger.
pub fn default(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();

    let active = pick_list::Style {
        text_color: palette.background.weakest.text,
        placeholder_color: palette.background.weakest.text.scale_alpha(0.6),
        handle_color: palette.background.weakest.text,
        background: palette.background.weakest.color.into(),
        border: border::rounded(CONTROL_RADIUS),
    };

    match status {
        pick_list::Status::Active => active,
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => pick_list::Style {
            background: palette.background.weaker.color.into(),
            ..active
        },
    }
}

/// The dropdown, matching the `iced_aw` menu.
pub fn default_menu(theme: &Theme) -> menu::Style {
    let palette = theme.extended_palette();

    menu::Style {
        background: palette.background.base.color.into(),
        // The overlay reuses this radius for the highlight of each item,
        // so it has to stay as tight as the one of a `button`.
        border: border::rounded(CONTROL_RADIUS),
        text_color: palette.background.base.text,
        selected_background: palette.primary.weak.color.into(),
        selected_text_color: palette.primary.weak.text,
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            offset: Vector::ZERO,
            blur_radius: 10.0,
        },
    }
}
