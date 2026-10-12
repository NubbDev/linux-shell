use std::time::Instant;

use iced::{
    Task,
    widget::operation::{AbsoluteOffset, scroll_to},
};

use crate::{animations::marquee::Marquees, event::AppEvent, view::components::marquee_content_id};

pub fn marquee(map: Option<&Marquees>) -> Task<AppEvent> {
    let mut tasks = vec![];
    if let Some(marquees) = map {
        let now = Instant::now();
        for (key, slot) in marquees.iter() {
            let overflow = slot.overflow();
            if overflow <= 0.0 {
                continue;
            }
            let id = marquee_content_id(key);
            let progress = slot.progress(now);

            let offset = AbsoluteOffset {
                x: progress * overflow,
                y: 0.0,
            };
            tasks.push(scroll_to(id, offset))
        }
    }

    Task::batch(tasks)
}
