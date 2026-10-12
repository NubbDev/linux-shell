use std::collections::{BTreeMap, HashSet};

// use hyprland::{
//     data::{Workspace, Workspaces}, event_listener::WorkspaceEventData, shared::{HyprData, WorkspaceType},
// };
use iced::futures::{SinkExt, channel::mpsc::Sender};
use serde::{Deserialize, Deserializer};
use typemap_rev::{TypeMap, TypeMapKey};

use crate::{backend::EventType, event::{HyprlandEvent, HyprlandReceived, HyprlandSend}};

// pub fn current() -> hyprland::Result<HyprWorkspaces> {
//     let workspaces = Workspaces::get()?;
//     let active_workspace_ids = super::monitor::active_workspace_ids()?
//         .into_iter()
//         .collect::<HashSet<_>>();

//     Ok(HyprWorkspaces(
//         (workspaces
//             .iter())
//             .map(|workspace| {
//                 let mut workspace = HyprWorkspace::from(workspace);
//                 workspace.is_active = active_workspace_ids.contains(&workspace.id);
//                 (
//                     workspace.id,
//                     workspace,
//                 )
//             })
//             .collect(),
//     ))
// }


pub async fn change(workspace_id: i32, sender: Sender<EventType>) {
    if let Some(workspace) = get_workspace_by_id(workspace_id).await {
        let hypr_event: HyprlandEvent = HyprlandReceived::WorkspaceSwitched { id: workspace.id().unwrap() }.into();
        handle_send(hypr_event, sender).await;
    }
}

pub async fn added(workspace_id: i32, sender: Sender<EventType>) {
    match fetch().await {
        Ok(workspaces) => match workspaces.iter().find(|w| w.id() == Some(workspace_id)) {
            Some(workspace) => {
                let position = workspaces.iter().filter(|w| w.info().monitor == workspace.info().monitor).position(|w| w.id() == workspace.id());
                let hypr_event: HyprlandEvent = HyprlandReceived::WorkspaceCreated {
                    id: workspace.id().unwrap(),
                    name: workspace.info().name.clone(),
                    monitor: workspace.info().monitor.clone(),
                    position: position.map(|p| p as i32).unwrap_or(-1),
                }.into();
                handle_send(hypr_event, sender).await;
            },
            None => {
                eprintln!("Workspace with id {} not found", workspace_id);
                return;
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch workspaces: {}", e);
            return;
        }
    }
}

pub async fn deleted(workspace_id: i32, sender: Sender<EventType>) {
    let hypr_event: HyprlandEvent = HyprlandReceived::WorkspaceRemoved { id: workspace_id }.into();
    handle_send(hypr_event, sender).await;
}

pub async fn moved(workspace_id: i32, monitor: String, sender: Sender<EventType>) {
    if let Some(workspace) = get_workspace_by_id(workspace_id).await {
        let hypr_event: HyprlandEvent = HyprlandReceived::WorkspaceMoved {
            id: workspace.id().unwrap(),
            monitor: monitor.clone()
        }.into();

        handle_send(hypr_event, sender).await;
    }
}

pub async fn renamed(workspace_id: i32, sender: Sender<EventType>) {
    if let Some(workspace) = get_workspace_by_id(workspace_id).await {
        let hypr_event: HyprlandEvent = HyprlandReceived::WorkspaceRenamed {
            id: workspace.id().unwrap(),
            name: workspace.info().name.clone()
        }.into();

        handle_send(hypr_event, sender).await;
    }
}

async fn get_workspace_by_id(workspace_id: i32) -> Option<Workspace> {
    match fetch().await {
        Ok(workspaces) => match workspaces.iter().find(|w| w.id() == Some(workspace_id)) {
            Some(workspace) => Some(workspace.clone()),
            None => {
                eprintln!("Workspace with id {} not found", workspace_id);
                return None;
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch workspaces: {}", e);
            return None;
        }
    }
}

async fn handle_send(event: HyprlandEvent, mut sender: Sender<EventType>) {
    if let Err(e) = sender.send(EventType::event(event)).await {
        eprintln!("Failed to send Hyprland event: {e}");
    }
}

#[derive(Debug, Clone)] // Represents a workspace in Hyprland (Address, Data)
pub struct HyprWorkspaces(pub BTreeMap<String, RawHyprWorkspace>);

impl TypeMapKey for HyprWorkspaces {
    type Value = Self;
}

impl super::super::Patch for HyprWorkspaces {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> iced::Task<super::super::AppEvent> {
        data.insert::<HyprWorkspaces>(*self);
        iced::Task::none()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawHyprWorkspace {
    address: String,
    #[serde(default)]
    id: Option<i32>,
    name: String,
    #[serde(rename = "type")]
    w_type: WorkspaceType,
    monitor: String,
    #[serde(rename = "monitorID")]
    monitor_id: u32,
    windows: u32,
    #[serde(rename = "hasfullscreen")]
    has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    last_window: String,
    #[serde(rename = "lastwindowtitle")]
    last_window_title: String,
    #[serde(rename = "ispersistent")]
    is_persistent: bool,
    #[serde(rename = "tiledLayout")]
    tiled_layout: String,
}

impl RawHyprWorkspace {
    pub fn id(&self) -> Option<i32> { self.id }
    pub fn is_special(&self) -> bool { matches!(self.w_type, WorkspaceType::Special) }
    pub fn is_regular(&self) -> bool { matches!(self.w_type, WorkspaceType::Regular) }
}

#[derive(Debug, Clone)]
pub enum WorkspaceType {
    Regular,
    Special,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "RawHyprWorkspace")]
pub enum Workspace {
    Regular {
        id: i32,
        info: WorkspaceInfo,
    },
    Special {
        info: WorkspaceInfo,
    },
}

impl Workspace {
    pub fn id(&self) -> Option<i32> {
        match self {
            Workspace::Regular { id, .. } => Some(*id),
            Workspace::Special { .. } => None,
        }
    }

    pub fn info(&self) -> &WorkspaceInfo {
        match self {
            Workspace::Regular { info, .. } => info,
            Workspace::Special { info } => info,
        }
    }
}

impl TryFrom<RawHyprWorkspace> for Workspace {
    type Error = String; // must impl Display for serde

    fn try_from(raw: RawHyprWorkspace) -> Result<Self, Self::Error> {
        let info = WorkspaceInfo {
            name: raw.name.clone(),
            address: raw.address.clone(),
            monitor: raw.monitor.clone(),
            monitor_id: raw.monitor_id,
            windows: raw.windows,
            has_fullscreen: raw.has_fullscreen,
            last_window: raw.last_window.clone(),
            last_window_title: raw.last_window_title.clone(),
            is_persistent: raw.is_persistent,
            tiled_layout: raw.tiled_layout.clone(),
        };
        match raw.w_type {
            WorkspaceType::Regular => Ok(Workspace::Regular {
                id: raw.id.ok_or_else(|| {
                    format!("regular workspace {} has no id", raw.name)
                })?,
                info,
            }),
            WorkspaceType::Special => Ok(Workspace::Special {
                info,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub name: String,
    pub address: String,
    pub monitor: String,
    pub monitor_id: u32,
    pub windows: u32,
    pub has_fullscreen: bool,
    pub last_window: String,
    pub last_window_title: String,
    pub is_persistent: bool,
    pub tiled_layout: String,
}

impl<'a> Deserialize<'a> for WorkspaceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "normal" => Ok(WorkspaceType::Regular),
            "special" => Ok(WorkspaceType::Special),
            _ => Err(serde::de::Error::custom(format!("Unknown workspace type: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawActiveWorkspace {
    pub address: String,
    pub id: u32,
    pub w_type: WorkspaceType,
    pub name: String,
}

async fn fetch() -> std::io::Result<Vec<Workspace>> {
    super::request("j/workspaces").await
}

pub fn handle_de<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let workspace: RawActiveWorkspace = Deserialize::deserialize(deserializer)?;
    Ok(workspace.id)
}
