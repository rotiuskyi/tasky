mod expandable;
pub mod hoverable;
mod menu;
mod menu_item;
mod select;

pub use expandable::*;
pub use hoverable::*;
pub use menu::*;
pub use menu_item::*;
pub use select::*;

/// The corner radius of the controls: buttons, fields, menus.
///
/// Surfaces are rounded more than that; see [`hoverable::card`].
const CONTROL_RADIUS: u32 = 2;

/// Implements `tag` and `state` of a [`Widget`] with the given type, which the
/// widget then reads back with `tree.state.downcast_ref::<State>()`.
///
/// [`Widget`]: iced::advanced::Widget
macro_rules! widget_state {
    ($state:ty) => {
        fn tag(&self) -> ::iced::advanced::widget::tree::Tag {
            ::iced::advanced::widget::tree::Tag::of::<$state>()
        }

        fn state(&self) -> ::iced::advanced::widget::tree::State {
            ::iced::advanced::widget::tree::State::new(<$state as Default>::default())
        }
    };
}

/// Implements the listed [`Widget`] methods by handing them over to the element
/// in the given field, the only child of the widget.
///
/// ```ignore
/// impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Badge<'_, ..> {
///     widget_state!(State);
///     delegate!(content: children, diff, size, size_hint, layout, operate, overlay);
///
///     fn update(..) { /* the reason this widget exists */ }
///     fn draw(..) { /* .. */ }
/// }
/// ```
///
/// The list is spelled out at every call so that a reader of the widget sees
/// what it forwards and what it takes over. Two things are expected of the
/// impl: its generics are named `Message`, `Theme` and `Renderer`, and its
/// `children` come from here as well, so that the content is `children[0]`.
///
/// [`Widget`]: iced::advanced::Widget
macro_rules! delegate {
    ($content:ident: $($method:ident),+ $(,)?) => {
        $(delegate!(@ $content $method);)+
    };

    (@ $content:ident children) => {
        fn children(&self) -> ::std::vec::Vec<::iced::advanced::widget::Tree> {
            ::std::vec![::iced::advanced::widget::Tree::new(&self.$content)]
        }
    };

    (@ $content:ident diff) => {
        fn diff(&self, tree: &mut ::iced::advanced::widget::Tree) {
            tree.diff_children(::std::slice::from_ref(&self.$content));
        }
    };

    (@ $content:ident size) => {
        fn size(&self) -> ::iced::Size<::iced::Length> {
            self.$content.as_widget().size()
        }
    };

    (@ $content:ident size_hint) => {
        fn size_hint(&self) -> ::iced::Size<::iced::Length> {
            self.$content.as_widget().size_hint()
        }
    };

    (@ $content:ident layout) => {
        fn layout(
            &mut self,
            tree: &mut ::iced::advanced::widget::Tree,
            renderer: &Renderer,
            limits: &::iced::advanced::layout::Limits,
        ) -> ::iced::advanced::layout::Node {
            self.$content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        }
    };

    (@ $content:ident operate) => {
        fn operate(
            &mut self,
            tree: &mut ::iced::advanced::widget::Tree,
            layout: ::iced::advanced::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn ::iced::advanced::widget::Operation,
        ) {
            self.$content
                .as_widget_mut()
                .operate(&mut tree.children[0], layout, renderer, operation);
        }
    };

    (@ $content:ident update) => {
        fn update(
            &mut self,
            tree: &mut ::iced::advanced::widget::Tree,
            event: &::iced::Event,
            layout: ::iced::advanced::Layout<'_>,
            cursor: ::iced::advanced::mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn ::iced::advanced::Clipboard,
            shell: &mut ::iced::advanced::Shell<'_, Message>,
            viewport: &::iced::Rectangle,
        ) {
            self.$content.as_widget_mut().update(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    };

    (@ $content:ident mouse_interaction) => {
        fn mouse_interaction(
            &self,
            tree: &::iced::advanced::widget::Tree,
            layout: ::iced::advanced::Layout<'_>,
            cursor: ::iced::advanced::mouse::Cursor,
            viewport: &::iced::Rectangle,
            renderer: &Renderer,
        ) -> ::iced::advanced::mouse::Interaction {
            self.$content.as_widget().mouse_interaction(
                &tree.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        }
    };

    (@ $content:ident draw) => {
        fn draw(
            &self,
            tree: &::iced::advanced::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &::iced::advanced::renderer::Style,
            layout: ::iced::advanced::Layout<'_>,
            cursor: ::iced::advanced::mouse::Cursor,
            viewport: &::iced::Rectangle,
        ) {
            self.$content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }
    };

    (@ $content:ident overlay) => {
        fn overlay<'w>(
            &'w mut self,
            tree: &'w mut ::iced::advanced::widget::Tree,
            layout: ::iced::advanced::Layout<'w>,
            renderer: &Renderer,
            viewport: &::iced::Rectangle,
            translation: ::iced::Vector,
        ) -> ::std::option::Option<
            ::iced::advanced::overlay::Element<'w, Message, Theme, Renderer>,
        > {
            self.$content.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            )
        }
    };
}

use {delegate, widget_state};
