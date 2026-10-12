use crate::{animations::marquee::Marquees, app::AppState, event::AppEvent};
use iced::Task;

mod anim;
mod layout;

pub fn handle(state: &mut AppState) -> Task<AppEvent> {
    let win_tasks = layout::window_bound(&mut state.windows, &state.layers);
    let display_tasks = layout::display(&state.displays);

    let marquee_tasks = layout::marquee_bounds(state.animations::<Marquees>());

    Task::batch(vec![win_tasks, display_tasks, marquee_tasks])
}

pub fn animate(state: &mut AppState) -> Task<AppEvent> {
    for anim in state.animations.iter_mut() {
        anim.update(&state.data);
    }

    let marquee_tasks = anim::marquee(state.animations::<Marquees>());

    Task::batch(vec![marquee_tasks])
}
