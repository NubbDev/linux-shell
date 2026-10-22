#[derive(Debug, Clone)]
pub enum HyprlandEvent {
    Send(HyprlandSend),
    Received(HyprlandReceived),
}
#[derive(Debug, Clone)]
pub enum HyprlandReceived {
    WorkspaceSwitched {
        id: i32,
    },
    WorkspaceCreated {
        id: i32,
        name: String,
        monitor: String,
        position: i32,
    },
    WorkspaceRemoved {
        id: i32,
    },
    WorkspaceMoved {
        id: i32,
        monitor: String,
    },
    WorkspaceRenamed {
        id: i32,
        name: String,
    },
    MonitorAdded {
        id: u32,
        name: String,
        dimensions: (u32, u32),
        position: (i32, i32),
        refresh_rate: f32,
        make: String,
        model: String,
    },
    MonitorRemoved {
        id: u32,
    },
    MonitorFocused {
        name: String,
        workspace: i32,
    },
}

#[derive(Debug, Clone)]
pub enum HyprlandSend {
    SwitchWorkspace(i32),
}

impl From<HyprlandSend> for HyprlandEvent {
    fn from(send: HyprlandSend) -> Self {
        HyprlandEvent::Send(send)
    }
}
impl From<HyprlandReceived> for HyprlandEvent {
    fn from(recv: HyprlandReceived) -> Self {
        HyprlandEvent::Received(recv)
    }
}
impl From<HyprlandEvent> for super::AppEvent {
    fn from(event: HyprlandEvent) -> Self {
        super::AppEvent::Hyprland(event)
    }
}
