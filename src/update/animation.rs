use std::collections::HashSet;

use iced::{Rectangle, Task};

use crate::{
    animations::{
        hover::Hovers,
        marquee::{MarqueeKey, Marquees},
    },
    app::AppState,
    event::{AnimEvent, AppEvent},
};

pub fn handle(state: &mut AppState, event: AnimEvent) -> Task<AppEvent> {
    match event {
        AnimEvent::MarqueeBounds(k, o, i) => handle_marquee(state, k, o, i),
        AnimEvent::Hover(id, is_hovering) => handle_hover(state, id, is_hovering),
        _ => {}
    };
    Task::none()
}

fn handle_marquee(state: &mut AppState, key: MarqueeKey, outer: Rectangle, inner: Rectangle) {
    if let Some(marquee) = state.animations_mut::<Marquees>() {
        marquee.set_size(key, inner.width, outer.width, 20.0);
    }
}

fn handle_hover(state: &mut AppState, id: iced::widget::Id, is_hovering: bool) {
    let hovers = match state.data.get_mut::<Hovers>() {
        Some(h) => h,
        None => {
            state.data.insert::<Hovers>(HashSet::new());
            state.data.get_mut::<Hovers>().unwrap()
        }
    };

    if is_hovering {
        hovers.insert(id);
    } else {
        hovers.remove(&id);
    }
}
