use hyprland::shared::WorkspaceType;

#[derive(Debug, Clone)]
pub enum HyprlandEvent {
    Send(HyprlandSend),
    Received(HyprlandReceived),
}
#[derive(Debug, Clone)]
pub enum HyprlandReceived {
    WorkspaceSwitched { id: i32 },
    WorkspaceCreated { id: i32, name: String, monitor: String, position: i32 },
    WorkspaceRemoved { id: i32 },
    WorkspaceMoved { id: i32, monitor: String },
    WorkspaceRenamed { id: i32, name: String },
    MonitorAdded { name: String },
    MonitorRemoved { name: String },
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
