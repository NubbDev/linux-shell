use iced::{Task, window::Id};

use crate::{
    app::AppState,
    event::{AppEvent, WindowEvent},
    view::WindowComponent,
};

mod bound;
mod layout;
pub fn handle(state: &mut AppState, event: WindowEvent) -> Task<AppEvent> {
    match event {
        WindowEvent::Open(id, info) => open_window(state, id, info),
        WindowEvent::BoundsChanged(id, bounds) => bound::computate(state, id, bounds),
        WindowEvent::ComputateLayout(display, group) => layout::computate(state, display, group),
        _ => Task::none(),
    }
}

fn open_window(state: &mut AppState, id: Id, info: Box<dyn WindowComponent>) -> Task<AppEvent> {
    state.windows.insert(id, info);
    Task::none()
}
