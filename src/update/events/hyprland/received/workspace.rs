use typemap_rev::TypeMap;

use crate::data::{HyprWorkspaces, Workspace};

pub fn get(data: &mut TypeMap) -> Option<&mut HyprWorkspaces> {
    data.get_mut::<Workspace>()
}
pub fn switched(workspaces: &mut HyprWorkspaces, current_id: i32) {
    workspaces.iter_mut().for_each(|(_, workspace)| {
        if workspace.id == current_id {
            // Set the current workspace to active
            workspace.is_active = true;
        } else if workspace.is_active {
            // Find the previous active workspace and set it to inactive
            workspace.is_active = false;
        }
    });
}
pub fn created(data: &mut TypeMap, id: i32, name: String, monitor: String, position: i32) {
    let new_workspace = Workspace {
        id,
        name,
        monitor,
        position,
        is_active: false,
    };

    if let Some(workspaces) = data.get_mut::<Workspace>() {
        workspaces.insert(id, new_workspace);
    } else {
        let mut workspaces = HyprWorkspaces::new();
        workspaces.insert(id, new_workspace);
        data.insert::<Workspace>(workspaces);
    }
}
pub fn removed(workspaces: &mut HyprWorkspaces, id: i32) {
    workspaces.remove(&id);
}

pub fn moved(workspaces: &mut HyprWorkspaces, id: i32, monitor: String) {
    if let Some(workspace) = workspaces.get_mut(&id) {
        workspace.monitor = monitor;
    }
}

pub fn renamed(workspaces: &mut HyprWorkspaces, id: i32, name: String) {
    if let Some(workspace) = workspaces.get_mut(&id) {
        workspace.name = name;
    }
}
