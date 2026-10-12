use hyprland::{event_listener::{AsyncEventListener, EventListener}, prelude::async_closure};
use iced::futures::{SinkExt, channel::mpsc::Sender};
use serde::de::DeserializeOwned;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::env;

use super::EventType;

mod event;

mod monitor;
mod workspace;
mod window;
mod screencast;

pub use workspace::{HyprWorkspaces};

use tokio::time::{sleep, Duration};

pub async fn run(sender: Sender<EventType>) {

    set_current_workspaces(sender.clone()).await;

    if let Ok(socket_path) = event_socket_path() &&
    let Ok(stream) = UnixStream::connect(socket_path).await {
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
                    eprintln!("Failed to parse Hyprland event: {}", e);
                }
            }
        }
    }
}

async fn set_current_workspaces(mut sender: Sender<EventType>) {
    // if let Ok(workspaces) = workspace::current() {
    //     if let Err(error) = sender.send(EventType::data(workspaces)).await {
    //         eprintln!("Failed to send Hyprland workspaces: {error}");
    //     }
    // }
}


fn hypr_dir() -> Result<String, String> {
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not set".to_string())?;
    let runtime = env::var("XDG_RUNTIME_DIR")
        .map_err(|_| "XDG_RUNTIME_DIR not set".to_string())?;
    Ok(format!("{runtime}/hypr/{sig}"))
}

pub fn event_socket_path() -> Result<String, String> {
    Ok(format!("{}/.socket2.sock", hypr_dir()?))
}

pub fn request_socket_path() -> Result<String, String> {
    Ok(format!("{}/.socket.sock", hypr_dir()?))
}

pub(super) async fn request<T: DeserializeOwned>(cmd: &str) -> std::io::Result<T> {
    let socket_path = request_socket_path().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    let mut stream = UnixStream::connect(socket_path).await?;
    // Hyprland's request protocol: command with no trailing newline is fine,
    // but a newline is harmless. Write, then half-close so Hyprland knows
    // we're done writing and will flush + close the reply side.
    stream.write_all(cmd.as_bytes()).await?;
    stream.shutdown().await?;    // <-- this is the key: shutdown write half

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
