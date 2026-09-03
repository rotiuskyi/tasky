//! An `iced_aw` menu bar dressed to match the menus of the app.
use iced::advanced::renderer;
use iced::{Theme, border};
use iced_aw::menu::{Item, MenuBar};
use iced_aw::style::{Status, menu_bar};

use crate::widgets::CONTROL_RADIUS;

/// Creates a [`MenuBar`] with the default styles, ready to be overridden.
pub fn menu_bar<'a, Message, Renderer>(
    roots: Vec<Item<'a, Message, Theme, Renderer>>,
) -> MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    MenuBar::new(roots).style(default)
}

/// The bar and its menus, matching the dropdown of a
/// [`select`](super::select::select).
pub fn default(theme: &Theme, status: Status) -> menu_bar::Style {
    menu_bar::Style {
        bar_border: border::rounded(CONTROL_RADIUS),
        menu_border: border::rounded(CONTROL_RADIUS),
        // The path is the highlight behind the hovered item; `iced_aw` keeps it
        // in a field of its own, so it does not follow the menu on its own.
        path_border: border::rounded(CONTROL_RADIUS),
        ..menu_bar::primary(theme, status)
    }
}
