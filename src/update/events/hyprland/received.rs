use hyprland::shared::WorkspaceType;
use iced::Task;
use crate::{app::AppState, event::{AppEvent, HyprlandReceived}};

pub fn handle(app_state: &mut AppState, event: HyprlandReceived) -> Task<AppEvent> {
    // if let Some(workspaces) =  app_state.data.get_mut::<HyprWorkspaces>() {
    //     match event {
    //         HyprlandReceived::WorkspaceSwitched { current } => switched_workspace(workspaces, current),
    //         HyprlandReceived::WorkspaceCreated { address, name, monitor, position } => created_workspace(workspaces, address, name, monitor, position),
    //         HyprlandReceived::WorkspaceRemoved { address } => removed_workspace(workspaces, address),
    //         HyprlandReceived::WorkspaceMoved { address, monitor } => moved_workspace(workspaces, address, monitor),
    //         HyprlandReceived::MonitorAdded { name } => todo!(),
    //         HyprlandReceived::MonitorRemoved { name } => todo!(),
    //     }
    // }

    return Task::none()
}

// fn switched_workspace(workspaces: &mut HyprWorkspaces, address: String) {
//     println!("Workspace switched: current={}", address);
//     // workspaces.0.iter_mut().for_each(|(_, workspace)| {
//     //     if workspace.id == current { // Set the current workspace to active
//     //         workspace.is_active = true;
//     //     } else if workspace.is_active { // Find the previous active workspace and set it to inactive
//     //         workspace.is_active = false;
//     //     }
//     // });

// }

// fn created_workspace(workspaces: &mut HyprWorkspaces, address: String, name: String, monitor: String, position: i32) {
//     println!("Workspace created: addr={}, name={:?}, monitor={}, position={}", address, name, monitor, position);
//     // let new_workspace = HyprWorkspace {
//     //     id,
//     //     name,
//     //     monitor,
//     //     windows: 0,
//     //     is_active: false,
//     // };
//     // workspaces.0.insert(id, new_workspace);
// }

// fn removed_workspace(workspaces: &mut HyprWorkspaces, address: String) {
//     println!("Workspace removed: addr={}", address);
//     // workspaces.0.remove(&id);
// }

// fn moved_workspace(workspaces: &mut HyprWorkspaces, address: String, monitor: String) {
//     println!("Workspace moved: addr={}, monitor={}", address, monitor);
//     // if let Some(workspace) = workspaces.0.get_mut(&id) {
//     //     workspace.monitor = monitor;
//     // }
// }
