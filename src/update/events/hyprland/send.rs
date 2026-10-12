use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use iced::Task;

use crate::{app::AppState, event::{AppEvent, HyprlandSend}};

pub fn handle(event: HyprlandSend) -> Task<AppEvent> {
    let new_hypr_event = match event {
        HyprlandSend::SwitchWorkspace(id) => DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id)),
    };
    if let Err(error) = Dispatch::call(new_hypr_event) {
        eprintln!("Failed to switch Hyprland workspace: {error}");
    }
    Task::none()
}
