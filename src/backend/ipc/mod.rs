use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::net::UnixListener,
    path::PathBuf,
    thread,
};

use iced::{
    Subscription,
    futures::{
        StreamExt,
        channel::mpsc::{Sender, unbounded},
        stream::BoxStream,
    },
};

use super::EventType;
use crate::event::AppEvent;

fn ipc_socket() {
    let (tx, mut rx) = unbounded::<EventType>();
    let socket_path = ipc_socket_path();

    thread::spawn(move || {
        let _ = fs::remove_file(&socket_path);
        let listener = match UnixListener::bind(&socket_path) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to socket {}: {}", socket_path.display(), e);
                return;
            }
        };
        println!("Starting IPC Socket");

        for stream in listener.incoming().flatten() {
            let reader = BufReader::new(stream);
            for line in reader.lines().flatten() {
                if let Some(event) = parse_ipc_command(&line) {
                    if let Err(e) = tx.unbounded_send(event) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
            }
        }
    });
}

fn ipc_socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    format!("{runtime_dir}/desktop.sock").into()
}

fn parse_ipc_command(line: &str) -> Option<EventType> {
    let command = line.trim();
    println!("Received IPC command: {}", command);
    None
}
