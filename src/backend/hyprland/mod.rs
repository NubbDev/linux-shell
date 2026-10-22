use iced::futures::{SinkExt, channel::mpsc::Sender};
use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::env;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::event::{HyprlandEvent, HyprlandReceived};

use super::EventType;

mod event;

mod monitor;
mod screencast;
mod window;
mod workspace;

use tokio::time::{Duration, sleep};

pub async fn run(sender: Sender<EventType>) {
    if let Err(e) = current_state(sender.clone()).await {
        eprintln!("Failed to fetch current state: {}", e);
    }

    if let Ok(socket_path) = event_socket_path()
        && let Ok(stream) = UnixStream::connect(socket_path).await
    {
        println!("Starting Hyprland event listener...");
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await.unwrap_or(0);
            if bytes_read == 0 {
                eprintln!("Hyprland socket closed");
                break;
            }
            match line.trim().parse::<event::RawEvent>() {
                Ok(raw_event) => event::handle(raw_event, sender.clone()).await,
                Err(e) => {
                    eprintln!("Failed Hyprland event parse: ({}) {}", line.trim(), e);
                }
            }
        }
    }
}

fn hypr_dir() -> Result<String, String> {
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not set".to_string())?;
    let runtime = env::var("XDG_RUNTIME_DIR").map_err(|_| "XDG_RUNTIME_DIR not set".to_string())?;
    Ok(format!("{runtime}/hypr/{sig}"))
}

pub fn event_socket_path() -> Result<String, String> {
    Ok(format!("{}/.socket2.sock", hypr_dir()?))
}

pub fn request_socket_path() -> Result<String, String> {
    Ok(format!("{}/.socket.sock", hypr_dir()?))
}

pub(super) async fn request<T: DeserializeOwned>(cmd: &str) -> std::io::Result<T> {
    let socket_path =
        request_socket_path().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut stream = UnixStream::connect(socket_path).await?;
    // Hyprland's request protocol: command with no trailing newline is fine,
    // but a newline is harmless. Write, then half-close so Hyprland knows
    // we're done writing and will flush + close the reply side.
    stream.write_all(cmd.as_bytes()).await?;
    stream.shutdown().await?; // <-- this is the key: shutdown write half

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    // Hyprland sometimes leaves a trailing newline; serde tolerates it,
    // but trimming is cleaner for logging.
    let trimmed = response.trim();
    serde_json::from_str(trimmed).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("failed to parse response {trimmed:?}: {e}"),
        )
    })
}

async fn handle_send(event: HyprlandEvent, mut sender: Sender<EventType>) {
    if let Err(e) = sender.send(EventType::event(event)).await {
        eprintln!("Failed to send Hyprland event: {e}");
    }
}

async fn current_state(sender: Sender<EventType>) -> Result<(), Box<dyn std::error::Error>> {
    let workspaces = workspace::fetch().await?;
    let monitors = monitor::fetch().await?;

    let workspaces_by_monitor = (workspaces.iter())
        .filter(|w| w.is_regular())
        .into_group_map_by(|w| w.info().monitor_id);

    for mut workspaces in workspaces_by_monitor.values().cloned() {
        workspaces.sort_by_key(|w| w.id());
        for workspace in workspaces.iter() {
            let position = workspaces.iter().position(|w| w.id() == workspace.id());
            let hypr_event: HyprlandEvent = HyprlandReceived::WorkspaceCreated {
                id: workspace.id().unwrap_or(-1),
                name: workspace.info().name.clone(),
                monitor: workspace.info().monitor.clone(),
                position: position.map(|p| p as i32).unwrap_or(-1),
            }
            .into();
            handle_send(hypr_event, sender.clone()).await;
        }
    }

    for monitor in monitors.iter() {
        let monitor_event: HyprlandEvent = HyprlandReceived::MonitorAdded {
            id: monitor.id,
            name: monitor.name.clone(),
            dimensions: (monitor.width, monitor.height),
            position: (monitor.x, monitor.y),
            refresh_rate: monitor.refresh_rate,
            make: monitor.make.clone(),
            model: monitor.model.clone(),
        }
        .into();

        let workspace_id = monitor.active_workspace;
        let workspace_event: HyprlandEvent = HyprlandReceived::MonitorFocused {
            name: monitor.name.clone(),
            workspace: workspace_id,
        }
        .into();
        handle_send(monitor_event, sender.clone()).await;
        handle_send(workspace_event, sender.clone()).await;
    }

    Ok(())
}
