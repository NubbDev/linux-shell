use iced::widget::Id;

mod marquee;
mod split_text;

pub use marquee::{marquee, marquee_content_id};
pub use split_text::SplitText;

use crate::event::{AnimEvent, AppEvent};

pub fn set_hover(id: &Id, value: bool) -> AppEvent {
    AppEvent::Animation(AnimEvent::Hover(id.clone(), value))
}
