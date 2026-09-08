//! A wrapper that styles its content while the cursor is over it.
//!
//! `container` in iced styles itself with a `Fn(&Theme) -> container::Style`,
//! so it cannot react to anything. [`Hoverable`] takes a `Fn(&Theme, Status)`
//! style instead and keeps its state in its own widget state, so the caller
//! needs neither a message nor a field in the application.
//!
//! Everything but that style is delegated to the content, so the wrapper is
//! invisible to the layout: size, padding and alignment stay the business of
//! whatever it wraps.
//!
//! Today [`Status`] only tells whether the cursor is over the widget; new
//! states are meant to be added to it as the widget grows.

use iced::advanced::layout::Layout;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Shell, Widget, mouse, renderer};
use iced::widget::container;
use iced::{Background, Element, Event, Rectangle, Theme, border};

use crate::widgets::{delegate, widget_state};

/// The current state of a [`Hoverable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The cursor is elsewhere.
    Idle,
    /// The cursor is over the widget.
    Hovered,
}

/// The style of a [`Hoverable`] for a given [`Status`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> container::Style + 'a>;

/// Creates a [`Hoverable`] wrapping the given content.
pub fn hoverable<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Hoverable<'a, Message, Theme, Renderer> {
    Hoverable::new(content)
}

/// A widget whose style depends on its [`Status`].
pub struct Hoverable<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    style: StyleFn<'a, Theme>,
}

impl<'a, Message, Theme, Renderer> Hoverable<'a, Message, Theme, Renderer> {
    /// Creates a [`Hoverable`] wrapping the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            style: Box::new(|_theme, _status| container::Style::default()),
        }
    }

    /// Sets the style of the [`Hoverable`].
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> container::Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }
}

/// The internal state of a [`Hoverable`].
#[derive(Debug, Default)]
struct State {
    is_hovered: bool,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Hoverable<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    widget_state!(State);
    delegate!(
        content: children, diff, size, size_hint, layout, operate, mouse_interaction, overlay
    );

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let state = tree.state.downcast_mut::<State>();
        let is_hovered = cursor.is_over(layout.bounds());

        // The style is only read while drawing, so a change of status has to
        // ask for a new frame explicitly.
        if is_hovered != state.is_hovered {
            state.is_hovered = is_hovered;
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if !bounds.intersects(viewport) {
            return;
        }

        let status = if tree.state.downcast_ref::<State>().is_hovered {
            Status::Hovered
        } else {
            Status::Idle
        };

        let style = (self.style)(theme, status);

        container::draw_background(renderer, &style, bounds);

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color.unwrap_or(renderer_style.text_color),
            },
            layout,
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Hoverable<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(hoverable: Hoverable<'a, Message, Theme, Renderer>) -> Self {
        Element::new(hoverable)
    }
}

/// A rounded surface that lights up when the cursor is over it.
pub fn card(theme: &Theme, status: Status) -> container::Style {
    let palette = theme.extended_palette();

    let background = match status {
        Status::Idle => palette.background.weakest.color.scale_alpha(0.5),
        Status::Hovered => palette.background.weakest.color,
    };

    container::Style {
        background: Some(background.into()),
        text_color: Some(palette.background.base.text),
        border: border::rounded(4),
        ..container::Style::default()
    }
}

/// Turns a plain `container` style into a status-aware one by swapping only the
/// background when the cursor is over the widget.
///
/// This is the escape hatch for reusing styles that already exist, including the
/// built-in ones:
///
/// ```ignore
/// hoverable(content).style(hovered_bg(container::bordered_box, |theme| {
///     theme.extended_palette().primary.weak.color.into()
/// }))
/// ```
#[allow(dead_code)] // part of the widget's public surface
pub fn hovered_bg<'a>(
    base: impl Fn(&Theme) -> container::Style + 'a,
    background: impl Fn(&Theme) -> Background + 'a,
) -> impl Fn(&Theme, Status) -> container::Style + 'a {
    move |theme, status| {
        let style = base(theme);

        match status {
            Status::Idle => style,
            Status::Hovered => container::Style {
                background: Some(background(theme)),
                ..style
            },
        }
    }
}
