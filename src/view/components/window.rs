use std::slice;

use iced::{
    Element, Length, Rectangle, Size,
    advanced::{
        self, Layout, Widget,
        layout::{Limits, Node},
        renderer,
        widget::{self, Tree},
    },
    widget::Id as WidgetId,
};

pub struct Window<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    id: Option<WidgetId>,
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Window<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            id: None,
            content: content.into(),
        }
    }

    pub fn id(mut self, id: WidgetId) -> Self {
        self.id = Some(id);
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Window<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(slice::from_ref(&self.content));
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        if let Some(id) = &self.id {
            operation.custom(Some(id), layout.bounds(), &mut ());
        }

        let Some(child_layout) = layout.children().next() else {
            return;
        };

        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                child_layout,
                renderer,
                operation,
            );
        });
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, _limits: &Limits) -> Node {
        let intrinsic_limits = Limits::NONE;

        let child = (self.content.as_widget_mut()).layout(
            &mut tree.children[0],
            renderer,
            &intrinsic_limits,
        );

        Node::with_children(child.size(), vec![child])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Window<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
{
    fn from(window: Window<'a, Message, Theme, Renderer>) -> Self {
        Element::new(window)
    }
}
pub fn window<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Window<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    Window::new(content)
}
