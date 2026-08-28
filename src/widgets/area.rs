//! A `container` that reacts to its own state.
//!
//! `container` in iced styles itself with a `Fn(&Theme) -> container::Style`,
//! so it cannot react to anything. [`Area`] is the same container with a
//! `Fn(&Theme, Status)` style instead: it keeps its state in its own widget
//! state, so the caller needs neither a message nor a field in the application.
//!
//! Today [`Status`] only tells whether the cursor is over the widget; new
//! states are meant to be added to it as the widget grows.

use iced::advanced::widget::{Operation, Tree, tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, renderer};
use iced::widget::container;
use iced::{
    Background, Element, Event, Length, Padding, Rectangle, Size, Theme, Vector, alignment, border,
};

/// The current state of a [`Area`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The cursor is elsewhere.
    Idle,
    /// The cursor is over the widget.
    Hovered,
}

/// The style of a [`Area`] for a given [`Status`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> container::Style + 'a>;

/// Creates a [`Area`] wrapping the given content.
pub fn area<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Area<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    Area::new(content)
}

/// A container whose style depends on its [`Status`].
pub struct Area<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    align_x: alignment::Horizontal,
    align_y: alignment::Vertical,
    clip: bool,
    style: StyleFn<'a, Theme>,
}

impl<'a, Message, Theme, Renderer> Area<'a, Message, Theme, Renderer> {
    /// Creates a [`Area`] wrapping the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self
    where
        Renderer: renderer::Renderer,
    {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Self {
            content,
            width: size.width.fluid(),
            height: size.height.fluid(),
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            padding: Padding::ZERO,
            align_x: alignment::Horizontal::Left,
            align_y: alignment::Vertical::Top,
            clip: false,
            style: Box::new(|_theme, _status| container::Style::default()),
        }
    }

    /// Sets the [`Padding`] of the [`Area`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Area`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Area`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Area`].
    pub fn max_width(mut self, max_width: impl Into<iced::Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`Area`].
    pub fn max_height(mut self, max_height: impl Into<iced::Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Aligns the contents of the [`Area`] horizontally.
    pub fn align_x(mut self, align_x: impl Into<alignment::Horizontal>) -> Self {
        self.align_x = align_x.into();
        self
    }

    /// Aligns the contents of the [`Area`] vertically.
    pub fn align_y(mut self, align_y: impl Into<alignment::Vertical>) -> Self {
        self.align_y = align_y.into();
        self
    }

    /// Sets whether the contents should be clipped to the bounds.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the style of the [`Area`].
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> container::Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }
}

/// The internal state of a [`Area`].
#[derive(Debug, Default)]
struct State {
    is_hovered: bool,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Area<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        container::layout(
            limits,
            self.width,
            self.height,
            self.max_width,
            self.max_height,
            self.padding,
            self.align_x,
            self.align_y,
            |limits| {
                self.content
                    .as_widget_mut()
                    .layout(&mut tree.children[0], renderer, limits)
            },
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

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
            layout.children().next().unwrap(),
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

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
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

        let Some(clipped_viewport) = bounds.intersection(viewport) else {
            return;
        };

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
            layout.children().next().unwrap(),
            cursor,
            if self.clip {
                &clipped_viewport
            } else {
                viewport
            },
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Area<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(area: Area<'a, Message, Theme, Renderer>) -> Self {
        Element::new(area)
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
/// area(content).style(hovered_bg(container::bordered_box, |theme| {
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
