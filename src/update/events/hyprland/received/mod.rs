use crate::{
    app::AppState,
    data::Monitor,
    event::{AppEvent, HyprlandReceived},
};
use iced::Task;
use typemap_rev::TypeMap;
mod monitor;
mod workspace;

pub fn handle(app_state: &mut AppState, event: HyprlandReceived) -> Task<AppEvent> {
    match event {
        HyprlandReceived::WorkspaceSwitched { id } => {
            if let Some(workspaces) = workspace::get(&mut app_state.data) {
                workspace::switched(workspaces, id);
            }
        }
        HyprlandReceived::WorkspaceCreated {
            id,
            name,
            monitor,
            position,
        } => workspace::created(&mut app_state.data, id, name, monitor, position),
        HyprlandReceived::WorkspaceRemoved { id } => {
            if let Some(workspaces) = workspace::get(&mut app_state.data) {
                workspace::removed(workspaces, id);
            }
        }
        HyprlandReceived::WorkspaceMoved { id, monitor } => {
            if let Some(workspaces) = workspace::get(&mut app_state.data) {
                workspace::moved(workspaces, id, monitor);
            }
        }
        HyprlandReceived::WorkspaceRenamed { id, name } => {
            if let Some(workspaces) = workspace::get(&mut app_state.data) {
                workspace::renamed(workspaces, id, name);
            }
        }
        HyprlandReceived::MonitorAdded {
            id,
            name,
            dimensions,
            position,
            refresh_rate,
            make,
            model,
        } => monitor::added(
            &mut app_state.data,
            id,
            name,
            dimensions,
            position,
            refresh_rate,
            make,
            model,
        ),
        HyprlandReceived::MonitorRemoved { id } => {
            if let Some(monitors) = monitor::get(&mut app_state.data) {
                monitor::removed(monitors, id);
            }
        }
        HyprlandReceived::MonitorFocused { name, workspace } => {
            focused_monitor(&mut app_state.data, name, workspace)
        }
    }

    return Task::none();
}
/*
 * This needs to update both the focused workspace of a specific monitor.
 * The monitor itself is not need to change, it is just the workspace that is focused on that monitor.
 */
fn focused_monitor(data: &mut TypeMap, name: String, workspace_id: i32) {
    if let Some(workspaces) = workspace::get(data) {
        for (_, workspace) in workspaces.iter_mut().filter(|(_, w)| w.monitor == name) {
            if workspace.id == workspace_id {
                workspace.is_active = true;
            } else if workspace.is_active {
                workspace.is_active = false;
            }
        }
    }
}
