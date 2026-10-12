use std::collections::HashMap;

use iced::Task;
use niri_ipc::{Event, Workspace, state::WorkspacesState};
use typemap_rev::{TypeMap, TypeMapKey};

use crate::event::AppEvent;

#[derive(Debug, Clone)]
pub struct NiriWorkspaces(pub HashMap<u64, Workspace>);

impl TypeMapKey for NiriWorkspaces {
    type Value = Self;
}

impl super::super::Patch for NiriWorkspaces {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(old_workspaces) = data.get_mut::<NiriWorkspaces>() {
            old_workspaces.clone_from(self.as_ref());
        } else {
            data.insert::<NiriWorkspaces>(*self);
        }
        Task::none()
    }
}

impl From<&WorkspacesState> for NiriWorkspaces {
    fn from(state: &WorkspacesState) -> Self {
        let workspaces = state
            .workspaces
            .iter()
            .map(|(id, ws)| (*id, ws.clone()))
            .collect::<HashMap<u64, Workspace>>();

        NiriWorkspaces(workspaces)
    }
}

pub fn is_event(event: &Event) -> bool {
    matches!(
        event,
        Event::WorkspaceActivated { .. }
            | Event::WorkspacesChanged { .. }
            | Event::WorkspaceUrgencyChanged { .. }
            | Event::WorkspaceActiveWindowChanged { .. }
    )
}
