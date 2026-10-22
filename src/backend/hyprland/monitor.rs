use crate::backend;
use crate::event::{HyprlandEvent, HyprlandReceived};

use super::workspace;
use hyprland::data::Monitors;
use hyprland::shared::HyprData;
use iced::futures::channel::mpsc::Sender;
use serde::Deserialize;

pub fn active_workspace_ids() -> hyprland::Result<Vec<i32>> {
    Ok(Monitors::get()?
        .iter()
        .map(|monitor| monitor.active_workspace.id)
        .collect())
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawHyprMonitor {
    pub id: u32,
    pub name: String,
    pub make: String,
    pub model: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
    #[serde(rename = "physicalWidth")]
    pub physical_width: i32,
    #[serde(rename = "physicalHeight")]
    pub physical_height: i32,
    #[serde(rename = "activeWorkspace")]
    #[serde(deserialize_with = "workspace::handle_de")]
    pub active_workspace: i32,
    pub reserved: [i32; 4],
    pub scale: f32,
    pub focused: bool,
    #[serde(rename = "availableModes")]
    pub available_modes: Vec<MonitorResolution>,
}

#[derive(Debug, Clone)]
struct MonitorResolution {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
}

impl<'de> Deserialize<'de> for MonitorResolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // format is expected to be "WIDTHxHEIGHT@REFRESH_RATEHz", e.g., "1920x1080@60Hz"
        let s = String::deserialize(deserializer)?.to_lowercase();
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "Invalid monitor resolution format: {}",
                s
            )));
        }
        let resolution_parts: Vec<&str> = parts[0].split('x').collect();
        if resolution_parts.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "Invalid resolution format: {}",
                parts[0]
            )));
        }
        let width = resolution_parts[0].parse::<u32>().map_err(|_| {
            serde::de::Error::custom(format!("Invalid width: {}", resolution_parts[0]))
        })?;
        let height = resolution_parts[1].parse::<u32>().map_err(|_| {
            serde::de::Error::custom(format!("Invalid height: {}", resolution_parts[1]))
        })?;

        let refresh_rate_str = parts[1].trim_end_matches("hz");
        let refresh_rate = refresh_rate_str.parse::<f32>().map_err(|_| {
            serde::de::Error::custom(format!("Invalid refresh rate: {}", refresh_rate_str))
        })?;

        Ok(MonitorResolution {
            width,
            height,
            refresh_rate,
        })
    }
}

pub async fn fetch() -> std::io::Result<Vec<RawHyprMonitor>> {
    super::request("j/monitors").await
}

pub async fn added(id: u32, sender: Sender<backend::EventType>) {
    if let Ok(monitors) = fetch().await {
        if let Some(monitor) = monitors.into_iter().find(|m| m.id == id) {
            let event: HyprlandEvent = HyprlandReceived::MonitorAdded {
                id: monitor.id,
                name: monitor.name,
                dimensions: (monitor.width, monitor.height),
                position: (monitor.x, monitor.y),
                refresh_rate: monitor.refresh_rate,
                make: monitor.make,
                model: monitor.model,
            }
            .into();
            super::handle_send(event, sender).await;
        }
    }
}

pub async fn removed(id: u32, sender: Sender<backend::EventType>) {
    let event: HyprlandEvent = HyprlandReceived::MonitorRemoved { id }.into();
    super::handle_send(event, sender).await;
}

pub async fn focused(monitor_name: String, workspace_id: i32, sender: Sender<backend::EventType>) {
    let event: HyprlandEvent = HyprlandReceived::MonitorFocused {
        name: monitor_name,
        workspace: workspace_id,
    }
    .into();
    super::handle_send(event, sender).await;
}

pub async fn current(sender: Sender<backend::EventType>) -> std::io::Result<()> {
    let monitors = fetch().await?;
    for monitor in monitors {
        let event: HyprlandEvent = HyprlandReceived::MonitorAdded {
            id: monitor.id,
            name: monitor.name,
            dimensions: (monitor.width, monitor.height),
            position: (monitor.x, monitor.y),
            refresh_rate: monitor.refresh_rate,
            make: monitor.make,
            model: monitor.model,
        }
        .into();
        super::handle_send(event, sender.clone()).await;
    }
    Ok(())
}
