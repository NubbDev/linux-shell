use std::thread;

use iced::{
    Subscription,
    futures::{
        StreamExt,
        channel::mpsc::{Sender, unbounded},
        stream::BoxStream,
    },
};
use niri_ipc::{
    socket::Socket,
    state::{EventStreamState, EventStreamStatePart},
};

use super::EventType;
use crate::event::AppEvent;

mod workspace;
pub use workspace::NiriWorkspaces;


fn niri() {
    let (tx, mut rx) = unbounded::<EventType>();

    println!("Starting Niri subscription thread");
    thread::spawn(move || {
        let Ok(mut socket) = Socket::connect() else {
            return;
        };
        if let Err(e) = socket.send(niri_ipc::Request::EventStream) {
            eprintln!("Failed to request event stream: {}", e);
            return;
        }

        let mut state = EventStreamState::default();
        let mut read_event = socket.read_events();

        while let Ok(event) = read_event() {
            state.apply(event.clone());

            if workspace::is_event(&event) {
                if let Err(e) =
                    tx.unbounded_send(EventType::data(NiriWorkspaces::from(&state.workspaces)))
                {
                    eprintln!("Failed to send workspaces: {}", e);
                    break;
                }
            }
        }
    });
}
