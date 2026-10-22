use iced::widget::Id;

mod marquee;
mod split_text;
mod window;

pub use marquee::{marquee, marquee_content_id};
pub use split_text::{SplitText, split_text};

pub use window::{Window, window};

use crate::event::{AnimEvent, AppEvent};

pub fn set_hover(id: &Id, value: bool) -> AppEvent {
    AppEvent::Animation(AnimEvent::Hover(id.clone(), value))
}
