//! A column that only lays out its body while it is expanded.
//!
//! The widget keeps `is_expanded` in its own state and flips it when the header
//! is clicked, so a plain [`expandable`] needs neither a message nor a field in
//! the application.
//!
//! A parent that has to drive it anyway — expand all, restore from storage —
//! overrides the flag with [`Expandable::expanded`] and follows the widget
//! through [`Expandable::on_action`]. While the flag is set, the internal state
//! is still kept up to date, so dropping the override resumes where the user
//! left off.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Shell, Widget, mouse, overlay, renderer};
use iced::{Element, Event, Length, Point, Rectangle, Size, Vector};

use crate::widgets::widget_state;

/// Something an [`Expandable`] did on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// The header was clicked; the body is now shown (`true`) or hidden.
    Toggled(bool),
}

/// Creates an [`Expandable`] that shows `body` under `header` once clicked.
pub fn expandable<'a, Message, Theme, Renderer>(
    header: impl Into<Element<'a, Message, Theme, Renderer>>,
    body: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Expandable<'a, Message, Theme, Renderer> {
    Expandable::new(header, body)
}

/// The index of the header among the children.
const HEADER: usize = 0;
/// The index of the body among the children.
const BODY: usize = 1;

/// A header that shows or hides its body when clicked.
pub struct Expandable<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    /// The header at [`HEADER`] and the body at [`BODY`]; a single array keeps
    /// them in step with their trees and their layout nodes.
    children: [Element<'a, Message, Theme, Renderer>; 2],
    is_expanded: Option<bool>,
    on_action: Option<Box<dyn Fn(Action) -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> Expandable<'a, Message, Theme, Renderer> {
    /// Creates an [`Expandable`] that shows `body` under `header` once clicked.
    pub fn new(
        header: impl Into<Element<'a, Message, Theme, Renderer>>,
        body: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            children: [header.into(), body.into()],
            is_expanded: None,
            on_action: None,
        }
    }

    /// Takes the state over from the widget.
    ///
    /// Clicks still toggle and still report an [`Action`], but only this value
    /// decides whether the body is shown.
    pub fn expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = Some(is_expanded);
        self
    }

    /// Sets the message to produce when the widget acts on its own.
    pub fn on_action(mut self, on_action: impl Fn(Action) -> Message + 'a) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }

    /// Whether the body is shown, with the override winning over the state.
    fn is_expanded(&self, tree: &Tree) -> bool {
        self.is_expanded
            .unwrap_or_else(|| tree.state.downcast_ref::<State>().is_expanded)
    }
}

/// The internal state of an [`Expandable`].
#[derive(Debug, Default)]
struct State {
    is_expanded: bool,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Expandable<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    widget_state!(State);

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        let header = self.children[HEADER].as_widget().size();
        let body = self.children[BODY].as_widget().size();

        Size {
            // The body has to fit once it is expanded, so a fluid one makes the
            // whole widget fluid.
            width: header.width.enclose(body.width),
            height: Length::Shrink,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let is_expanded = self.is_expanded(tree);

        let (header, body) = self.children.split_at_mut(BODY);
        let (header_tree, body_tree) = tree.children.split_at_mut(BODY);

        let header = header[0]
            .as_widget_mut()
            .layout(&mut header_tree[0], renderer, limits);
        let header_size = header.size();

        // A collapsed body is never laid out; it keeps an empty node so that
        // the children of the layout stay indexed like the ones of the tree.
        if !is_expanded {
            return Node::with_children(header_size, vec![header, Node::new(Size::ZERO)]);
        }

        let body = body[0]
            .as_widget_mut()
            .layout(
                &mut body_tree[0],
                renderer,
                &limits.shrink(Size::new(0.0, header_size.height)),
            )
            .move_to(Point::new(0.0, header_size.height));
        let body_size = body.size();

        Node::with_children(
            Size::new(
                header_size.width.max(body_size.width),
                header_size.height + body_size.height,
            ),
            vec![header, body],
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let is_expanded = self.is_expanded(tree);

        operation.traverse(&mut |operation| {
            for ((child, tree), layout) in self.children[..shown(is_expanded)]
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(tree, layout, renderer, operation);
            }
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
        let is_expanded = self.is_expanded(tree);
        let header_bounds = header_layout(layout).bounds();

        for ((child, tree), layout) in self.children[..shown(is_expanded)]
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }

        // Whatever the header holds — a checkbox, the menu of a task — comes
        // first; only a click that nothing claimed toggles the body.
        if shell.is_event_captured() {
            return;
        }

        let is_clicked = matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        ) && cursor.is_over(header_bounds);

        if !is_clicked {
            return;
        }

        let is_expanded = !is_expanded;
        tree.state.downcast_mut::<State>().is_expanded = is_expanded;

        if let Some(on_action) = &self.on_action {
            shell.publish(on_action(Action::Toggled(is_expanded)));
        }

        shell.capture_event();
        // The body joins or leaves the layout, and the new one has to be drawn.
        shell.invalidate_layout();
        shell.request_redraw();
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let interaction = self.children[..shown(self.is_expanded(tree))]
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, tree), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default();

        // The header is the button of this widget, so it says so.
        if interaction == mouse::Interaction::None && cursor.is_over(header_layout(layout).bounds())
        {
            return mouse::Interaction::Pointer;
        }

        interaction
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, tree), layout) in self.children[..shown(self.is_expanded(tree))]
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let shown = shown(self.is_expanded(tree));

        overlay::from_children(
            &mut self.children[..shown],
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

/// How many children take part in a pass: the header alone, or both.
fn shown(is_expanded: bool) -> usize {
    if is_expanded { BODY + 1 } else { HEADER + 1 }
}

/// The layout of the header, which every pass needs.
fn header_layout(layout: Layout<'_>) -> Layout<'_> {
    layout.children().next().expect("the header is laid out")
}

impl<'a, Message, Theme, Renderer> From<Expandable<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(expandable: Expandable<'a, Message, Theme, Renderer>) -> Self {
        Element::new(expandable)
    }
}
