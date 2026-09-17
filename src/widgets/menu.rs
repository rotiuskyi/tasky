use iced::Element;
use iced_aw::menu::MenuBar;

use crate::widgets::menu_item::MenuItem;

/// Emits [`Message`]; the parent converts it into its own message and tells
/// instances apart via the mapping, e.g.
/// `menu(menu_item("File"), &[menu_item("Open")]).map(Message::FileMenu)`.
pub fn menu<'a, Message: Clone + 'a>(
    root: MenuItem<Message>,
    items: Vec<MenuItem<Message>>,
) -> Element<'a, Message> {
    MenuBar::new(vec![root.with_menu(items).view_root()]).into()
}
