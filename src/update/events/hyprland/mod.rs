use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use iced::Task;

use crate::{
    app::AppState,
    event::{AppEvent, HyprlandEvent},
};

mod received;
mod send;

pub fn handle(app_state: &mut AppState, event: HyprlandEvent) -> Task<AppEvent> {
    match event {
        HyprlandEvent::Received(recv) => received::handle(app_state, recv),
        HyprlandEvent::Send(send) => send::handle(send),
    }
}
