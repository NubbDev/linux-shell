use iced::{
    Color, Element, Font, Length, Pixels, Rectangle, Size,
    advanced::{
        self, Layout, Text, Widget,
        layout::{self, Limits, Node},
        mouse, renderer,
        text::{Paragraph, paragraph},
        widget::{
            self, Tree,
            tree::{self, Tag},
        },
    },
    alignment::Vertical,
    widget::text::{Alignment, LineHeight, Shaping, State, Wrapping},
};
use std::marker::PhantomData;

pub struct SplitText<'a, Theme, Renderer>
where
    Renderer: advanced::text::Renderer,
{
    content: String,
    split_x: f32,
    width: Length,
    height: Length,
    size: Option<Pixels>,
    line_height: LineHeight,
    color_before: Color,
    color_after: Color,
    font: Option<Font>,
    align_y: Vertical,
    align_x: Alignment,
    shaping: Shaping,
    wrapping: Wrapping,
    _phantom: PhantomData<(&'a (), Theme, Renderer)>,
}

impl<'a, Theme, Renderer> SplitText<'a, Theme, Renderer>
where
    Renderer: advanced::text::Renderer<Font = iced::Font> + 'a,
{
    pub fn new(content: impl Into<String>, split_x: f32) -> Self {
        Self {
            content: content.into(),
            split_x,
            width: Length::Shrink,
            height: Length::Shrink,
            line_height: LineHeight::default(),
            color_before: Color::BLACK,
            color_after: Color::BLACK,
            size: None,
            font: None,
            align_y: Vertical::Center,
            align_x: Alignment::Left,
            shaping: Shaping::Basic,
            wrapping: Wrapping::None,
            _phantom: PhantomData,
        }
    }

    pub fn color_before(mut self, color: Color) -> Self {
        self.color_before = color;
        self
    }

    pub fn color_after(mut self, color: Color) -> Self {
        self.color_after = color;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn vertical_alignment(mut self, alignment: Vertical) -> Self {
        self.align_y = alignment;
        self
    }

    pub fn horizontal_alignment(mut self, alignment: Alignment) -> Self {
        self.align_x = alignment;
        self
    }

    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    pub fn line_height(mut self, line_height: LineHeight) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl<'a, Message, Theme, R> Widget<Message, Theme, R> for SplitText<'a, Theme, R>
where
    R: advanced::text::Renderer<Font = iced::Font> + 'a,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn tag(&self) -> Tag {
        Tag::of::<paragraph::Plain<R::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(paragraph::Plain::<R::Paragraph>::default())
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &R, limits: &Limits) -> Node {
        // Measure text intrinsically (equivalent to QML's invisible sizer)
        let paragraph = tree.state.downcast_mut::<State<R::Paragraph>>();

        layout::sized(limits, self.width, self.height, |l| {
            let _ = paragraph.update(Text {
                content: self.content.as_str(),
                bounds: l.max(),
                size: self.size.unwrap_or(renderer.default_size()),
                line_height: self.line_height,
                font: self.font.unwrap_or(renderer.default_font()),
                align_x: self.align_x,
                align_y: self.align_y,
                shaping: self.shaping,
                wrapping: self.wrapping,
            });

            paragraph.min_bounds()
        })
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut R,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<R::Paragraph>>();
        let paragraph = state.raw();
        let bounds = layout.bounds();

        // 1. Calculate correct anchor position based on alignment
        let anchor = bounds.anchor(
            paragraph.min_bounds(),
            paragraph.align_x(),
            paragraph.align_y(),
        );
        let clamped_split = self.split_x.clamp(0.0, bounds.width);

        // 2. Draw left portion (Filled bar progress area with `color_before`)
        if clamped_split > 0.0 {
            let left_clip = Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: clamped_split,
                height: bounds.height,
            };

            renderer.with_layer(left_clip, |r| {
                r.fill_paragraph(paragraph, anchor, self.color_before, *viewport);
            });
        }

        // 2. Draw right portion (Remaining background area with `color_after`)
        if clamped_split < bounds.width {
            let right_clip = Rectangle {
                x: bounds.x + clamped_split,
                y: bounds.y,
                width: bounds.width - clamped_split,
                height: bounds.height,
            };

            renderer.with_layer(right_clip, |r| {
                r.fill_paragraph(paragraph, anchor, self.color_after, *viewport);
            });
        }
    }
}

impl<'a, Message: 'a, Theme: 'a, R: advanced::text::Renderer<Font = iced::Font> + 'a>
    From<SplitText<'a, Theme, R>> for Element<'a, Message, Theme, R>
{
    fn from(widget: SplitText<'a, Theme, R>) -> Self {
        Self::new(widget)
    }
}
pub fn split_text<'a, Theme, R>(content: impl Into<String>, split_x: f32) -> SplitText<'a, Theme, R>
where
    R: advanced::text::Renderer<Font = iced::Font> + 'a,
{
    SplitText::new(content, split_x)
}
