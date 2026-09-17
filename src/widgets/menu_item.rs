use std::rc::Rc;

use iced::advanced::text::{self as core_text, Paragraph as _};
use iced::alignment::Vertical;
use iced::widget::{Button, Row, Text, button, text};
use iced::{Element, Font, Length, Pixels, Renderer, Size, Theme};

use iced_aw::menu::{Item, Menu};

/// Builds the icon anew on each view, already styled, e.g. `|| icons::trash().style(text::danger)`.
type IconFn = Rc<dyn Fn() -> Text<'static>>;

/// Must match the application's default text size (iced's default is 16).
const TEXT_SIZE: Pixels = Pixels(16.0);
const PADDING: f32 = 2.0;
const SPACING: f32 = 8.0;

/// What sits between the side icons.
#[derive(Clone)]
enum Content {
    Label(String),
    Icon(IconFn),
}

#[derive(Clone)]
pub struct MenuItem<Message> {
    content: Content,
    icon_left: Option<IconFn>,
    icon_right: Option<IconFn>,
    on_press: Option<Message>,
    items: Vec<MenuItem<Message>>,
}

pub fn menu_item<Message>(label: impl Into<String>) -> MenuItem<Message> {
    new(Content::Label(label.into()))
}

/// An entry showing only an icon, e.g. the `⋮` trigger of a menu.
pub fn menu_item_icon<Message>(icon_fn: impl Fn() -> Text<'static> + 'static) -> MenuItem<Message> {
    new(Content::Icon(Rc::new(icon_fn)))
}

fn new<Message>(content: Content) -> MenuItem<Message> {
    MenuItem {
        content,
        icon_left: None,
        icon_right: None,
        on_press: None,
        items: Vec::new(),
    }
}

impl<Message: Clone> MenuItem<Message> {
    pub fn icon_left(mut self, icon_fn: impl Fn() -> Text<'static> + 'static) -> Self {
        self.icon_left = Some(Rc::new(icon_fn));
        self
    }

    pub fn icon_right(mut self, icon_fn: impl Fn() -> Text<'static> + 'static) -> Self {
        self.icon_right = Some(Rc::new(icon_fn));
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Opens a submenu of `items` when hovered or pressed.
    ///
    /// Submenus can be nested to any depth:
    ///
    /// ```ignore
    /// menu_item("File").with_menu(&[
    ///     menu_item("Open").on_press(Event::Open),
    ///     menu_item("Export").with_menu(&[
    ///         menu_item("PNG").on_press(Event::ExportPng),
    ///         menu_item("SVG").on_press(Event::ExportSvg),
    ///     ]),
    /// ])
    /// ```
    pub fn with_menu(mut self, items: Vec<MenuItem<Message>>) -> Self {
        self.items = items.to_vec();
        self
    }

    /// An entry inside a [`Menu`], stretched to the menu's width.
    pub fn view<'a>(self) -> Item<'a, Message, Theme, Renderer>
    where
        Message: 'a,
    {
        self.item(Length::Fill)
    }

    /// A root entry of a `MenuBar`, as wide as its content.
    pub fn view_root<'a>(self) -> Item<'a, Message, Theme, Renderer>
    where
        Message: 'a,
    {
        self.item(Length::Shrink)
    }

    fn item<'a>(mut self, width: Length) -> Item<'a, Message, Theme, Renderer>
    where
        Message: 'a,
    {
        let items = std::mem::take(&mut self.items);
        let button = self.button(width).width(width);

        if items.is_empty() {
            return Item::new(button);
        }

        // `iced_aw`'s `Menu` can't derive its width with `Length::Shrink`: `Fill`
        // items only stretch to the widest non-`Fill` sibling, so all-`Fill` items
        // collapse.
        let menu_width = items.iter().map(Self::content_width).fold(0.0, f32::max);

        let menu = Menu::new(items.into_iter().map(Self::view).collect())
            .width(menu_width)
            .close_on_background_click(true);

        Item::with_menu(button, menu)
    }

    /// Width the entry needs to fit on one line.
    ///
    /// Icons are assumed to be square glyphs of [`TEXT_SIZE`].
    fn content_width(&self) -> f32 {
        let content = match &self.content {
            Content::Label(label) => {
                <Renderer as core_text::Renderer>::Paragraph::with_text(core_text::Text {
                    content: label.as_str(),
                    bounds: Size::INFINITE,
                    size: TEXT_SIZE,
                    line_height: core_text::LineHeight::default(),
                    font: Font::DEFAULT,
                    align_x: core_text::Alignment::Default,
                    align_y: Vertical::Top,
                    shaping: core_text::Shaping::default(),
                    wrapping: core_text::Wrapping::default(),
                })
                .min_width()
                .ceil()
            }
            Content::Icon(_) => TEXT_SIZE.0,
        };

        let icons = [&self.icon_left, &self.icon_right]
            .into_iter()
            .flatten()
            .count() as f32;

        content + icons * (TEXT_SIZE.0 + SPACING) + 2.0 * PADDING + button::DEFAULT_PADDING.x()
    }

    fn button<'a>(self, content_width: Length) -> Button<'a, Message, Theme, Renderer>
    where
        Message: 'a,
    {
        let content = match self.content {
            Content::Label(label) => text(label),
            Content::Icon(icon) => icon(),
        }
        .width(content_width);

        // Only the parts that are present, so a missing icon leaves no gap.
        let parts: [Option<Element<'a, Message>>; 3] = [
            self.icon_left.map(|icon| icon().into()),
            Some(content.into()),
            self.icon_right.map(|icon| icon().into()),
        ];

        button(Row::with_children(parts.into_iter().flatten()).spacing(SPACING))
            .style(button::subtle)
            .on_press_maybe(self.on_press)
    }
}
