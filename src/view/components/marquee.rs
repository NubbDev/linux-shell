use iced::{
    Renderer, Theme,
    widget::{
        Container, Id, Scrollable,
        scrollable::{Direction, Scrollbar},
    },
};
use iced_runtime::core::Element;

use crate::animations::marquee::MarqueeKey;

pub fn marquee<'a, Message: 'a>(
    id: &MarqueeKey,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message> {
    let scrollbar = Scrollbar::new().width(0).scroller_width(0);
    let content = Scrollable::new(content)
        .id(marquee_content_id(id))
        .direction(Direction::Horizontal(scrollbar));

    Container::new(content)
        .id(marquee_container_id(id))
        .clip(true)
}

fn marquee_container_id(key: &MarqueeKey) -> Id {
    Id::from(format!("marquee-container-{:?}-{}", key.0, key.1))
}

pub fn marquee_content_id(key: &MarqueeKey) -> Id {
    Id::from(format!("marquee-content-{:?}-{}", key.0, key.1))
}
