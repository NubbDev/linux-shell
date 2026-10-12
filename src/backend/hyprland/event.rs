use std::str::FromStr;

use iced::futures::channel::mpsc::Sender;

use crate::backend;

use super::{screencast, workspace, monitor, window};

#[derive(Debug, Clone)]
pub enum RawEvent {
    WorkspaceSwitched { id: i32 },
    FocusedMonitor { name: String, workspace_id: i32 },
    ActiveWindow { address: String },
    FullscreenWindow(bool),
    MonitorRemoved { id: i32 },
    MonitorAdded { id: i32 },
    WorkspaceCreated { id: i32 },
    WorkspaceRemoved { id: i32 },
    WorkspaceMoved { id: i32, monitor: String },
    WorkspaceRenamed { id: i32, name: String },
    SpecialActivated { id: Option<i32>, monitor: String },
    LayoutActivated { keyboard_name: String, layout_name: String },
    WindowOpened { address: String, workspace_name: String, class: String, title: String },
    WindowClosed { address: String },
    WindowKilled { address: String },
    WindowMoved { address: String, workspace_id: i32 },
    LayerOpened { namespace: String },
    LayerClosed { namespace: String },
    FloatingToggled { address: String, is_floating: bool },
    WindowUrgent { address: String },
    ScreenCasting { state: bool, owner: screencast::ScreenCastOwner },
    WindowTitleChanged { address: String, title: String },
    GroupToggled { state: bool, address: String },
    GroupMovedInto(String),
    GroupMovedOut(String),
    GroupIgnoredLocked(bool),
    GroupLocked(bool),
    ConfigReloaded,
    WindowPinned { address: String, is_pinned: bool },
    WindowMinimized { address: String, is_minimized: bool },
    SystemBell(String),
    Unknown(String),
}

impl FromStr for RawEvent {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim_end_matches(['\n', '\r']);
        let Some((name, data)) = line.split_once(">>") else {
            return Err(format!("Invalid event format: {}", line).into());
        };

        let event = match name {
            "workspacev2" => {
                let id = data.split(',').next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::WorkspaceSwitched { id }
            },
            "focusedmonv2" => {
                let mut parts = data.split(',');
                let name = parts.next().ok_or("Missing monitor name")?.to_string();
                let workspace_id = parts.next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::FocusedMonitor { name, workspace_id }
            }
            "activewindowv2" => {
                let address = data.to_string();
                RawEvent::ActiveWindow { address }
            }
            "fullscreen" => {
                let is_fullscreen = data == "1";
                RawEvent::FullscreenWindow(is_fullscreen)
            }
            "monitorremovedv2" => {
                let id = data.split(',').next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::MonitorRemoved { id }
            }
            "monitoraddedv2" => {
                let id = data.split(',').next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::MonitorAdded { id }
            }
            "createworkspacev2" => {
                let id = data.split(',').next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::WorkspaceCreated { id }
            }
            "destroyworkspacev2" => {
                let id = data.split(',').next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::WorkspaceRemoved { id }
            }
            "moveworkspacev2" => {
                let mut parts = data.split(',');
                let id = parts.next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                let _ = parts.next().ok_or("Missing workspace name")?.to_string();
                let monitor = parts.next().ok_or("Missing monitor name")?.to_string();
                RawEvent::WorkspaceMoved { id, monitor }
            }
            "renameworkspace" => {
                let mut parts = data.split(',');
                let id = parts.next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                let name = parts.next().ok_or("Missing new workspace name")?.to_string();
                RawEvent::WorkspaceRenamed { id, name }
            }
            "activespecialv2" => {
                let mut parts = data.split(',');
                let id = parts.next().map(|s| s.parse::<i32>().ok()).flatten();
                let _ = parts.next().ok_or("Missing workspace name")?.to_string();
                let monitor = parts.next().ok_or("Missing monitor name")?.to_string();
                RawEvent::SpecialActivated { id, monitor }
            }
            "activelayout" => {
                let mut parts = data.split(',');
                let keyboard_name = parts.next().ok_or("Missing keyboard name")?.to_string();
                let layout_name = parts.next().ok_or("Missing layout name")?.to_string();
                RawEvent::LayoutActivated { keyboard_name, layout_name }
            }
            "openwindow" => {
                let mut parts = data.split(',');
                let address = parts.next().ok_or("Missing window address")?.to_string();
                let workspace_name = parts.next().ok_or("Missing workspace name")?.to_string();
                let class = parts.next().ok_or("Missing window class")?.to_string();
                let title = parts.next().ok_or("Missing window title")?.to_string();
                RawEvent::WindowOpened { address, workspace_name, class, title }
            }
            "closewindow" => {
                let address = data.to_string();
                RawEvent::WindowClosed { address }
            }
            "kill" => {
                let address = data.to_string();
                RawEvent::WindowKilled { address }
            }
            "movewindowv2" => {
                let mut parts = data.split(',');
                let address = parts.next().ok_or("Missing window address")?.to_string();
                let workspace_id = parts.next().ok_or("Missing workspace ID")?.parse::<i32>()?;
                RawEvent::WindowMoved { address, workspace_id }
            }
            "openlayer" => {
                let namespace = data.to_string();
                RawEvent::LayerOpened { namespace }
            }
            "closelayer" => {
                let namespace = data.to_string();
                RawEvent::LayerClosed { namespace }
            }
            "submap" => RawEvent::Unknown(name.into()),
            "changefloatingmode" => {
                let mut parts = data.split(',');
                let address = parts.next().ok_or("Missing window address")?.to_string();
                let is_floating = parts.next().ok_or("Missing floating state")? == "1";
                RawEvent::FloatingToggled { address, is_floating }
            }
            "urgent" => {
                let address = data.to_string();
                RawEvent::WindowUrgent { address }
            }
            "screencastv2" => {
                let mut parts = data.split(',');
                let state = parts.next().ok_or("Missing screencast state")? == "1";
                let owner_str = parts.next().ok_or("Missing screencast owner")?;
                let owner = match owner_str {
                    "monitor" => screencast::ScreenCastOwner::Monitor(parts.next().ok_or("Missing screencast name")?.to_string()),
                    "window" => screencast::ScreenCastOwner::Window(parts.next().ok_or("Missing screencast name")?.to_string()),
                    "region" => screencast::ScreenCastOwner::Region,
                    _ => return Err(format!("Unknown screencast owner: {}", owner_str).into()),
                };
                RawEvent::ScreenCasting { state, owner }
            }
            "windowtitlev2" => {
                let mut parts = data.split(',');
                let address = parts.next().ok_or("Missing window address")?.to_string();
                let title = parts.next().ok_or("Missing new window title")?.to_string();
                RawEvent::WindowTitleChanged { address, title }
            }
            "togglegroup" => {
                let mut parts = data.split(',');
                let state = parts.next().ok_or("Missing group state")? == "1";
                let address = parts.next().ok_or("Missing window address")?.to_string();
                RawEvent::GroupToggled { state, address }
            }
            "moveintogroup" => {
                let address = data.to_string();
                RawEvent::GroupMovedInto(address)
            }
            "moveoutofgroup" => {
                let address = data.to_string();
                RawEvent::GroupMovedOut(address)
            }
            "ignoregrouplock" => {
                let state = data == "1";
                RawEvent::GroupIgnoredLocked(state)
            }
            "lockgroups" => {
                let state = data == "1";
                RawEvent::GroupLocked(state)
            }
            "configreloaded" => RawEvent::ConfigReloaded,
            "pin" => {
                let mut parts = data.split(',');
                let address = parts.next().ok_or("Missing window address")?.to_string();
                let is_pinned = parts.next().ok_or("Missing pinned state")? == "1";
                RawEvent::WindowPinned { address, is_pinned }
            }
            "minimized" => {
                let mut parts = data.split(',');
                let address = parts.next().ok_or("Missing window address")?.to_string();
                let is_minimized = parts.next().ok_or("Missing minimized state")? == "1";
                RawEvent::WindowMinimized { address, is_minimized }
            }
            "bell" => {
                let address = data.to_string();
                RawEvent::SystemBell(address)
            }
            _ => RawEvent::Unknown(name.to_string()),
        };

        Ok(event)
    }
}

pub async fn handle(event: RawEvent, mut sender: Sender<backend::EventType>) {
    match event {
        RawEvent::WorkspaceCreated { id } => workspace::added(id, sender).await,
        RawEvent::WorkspaceRemoved { id } => workspace::deleted(id, sender).await,
        RawEvent::WorkspaceSwitched { id } => workspace::change(id, sender).await,
        RawEvent::WorkspaceMoved { id, monitor } => workspace::moved(id, monitor, sender).await,
        RawEvent::WorkspaceRenamed { id, name } => workspace::renamed(id, sender).await,
        _ => {}
    }
}
