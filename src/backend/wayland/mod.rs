mod output;
pub use output::DisplayOutput;
use std::{hash::Hash, thread};

use iced::{
    Subscription,
    futures::{
        SinkExt, StreamExt,
        channel::mpsc::{Sender, unbounded},
        stream::BoxStream,
    },
};
use wayland_client::Connection;

use crate::event::{AppEvent, DisplayEvent};

pub struct ConnectionWrapper(pub Connection);

impl Hash for ConnectionWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        "wayland".hash(state);
    }
}

pub fn subscription(conn: Connection) -> Subscription<AppEvent> {
    Subscription::run_with(ConnectionWrapper(conn), wayland)
}

fn wayland(conn: &ConnectionWrapper) -> BoxStream<'static, AppEvent> {
    let connection = conn.0.clone();
    let (tx, mut rx) = unbounded::<AppEvent>();

    println!("Connected to Wayland server");
    thread::spawn(move || {
        let display = connection.display();
        let mut event_queue = connection.new_event_queue::<output::OutputTracker>();
        let qh = event_queue.handle();
        let _registry = display.get_registry(&qh, ());
        let mut output_tracker = output::OutputTracker::default();
        let mut known_outputs: Vec<u32> = Vec::new();

        loop {
            if event_queue.blocking_dispatch(&mut output_tracker).is_err() {
                break;
            }

            for output in output_tracker.0.iter() {
                if !known_outputs.contains(&output.id()) {
                    known_outputs.push(output.id());
                    if let Err(e) = tx
                        .unbounded_send(AppEvent::Display(DisplayEvent::NewDisplay(output.clone())))
                    {
                        eprintln!("Failed to send new display event: {}", e);
                    }
                } else {
                    if let Err(e) = tx.unbounded_send(AppEvent::Display(
                        DisplayEvent::UpdateDisplay(output.clone()),
                    )) {
                        eprintln!("Failed to send new display event: {}", e);
                    }
                }
            }

            if output_tracker.0.len() < known_outputs.len() {
                let removed_outputs: Vec<u32> = known_outputs
                    .iter()
                    .filter(|id| !output_tracker.0.iter().any(|output| output.id() == **id))
                    .map(|id| *id)
                    .collect();

                for removed_id in removed_outputs {
                    // println!("Display removed: {}", removed_id);
                    known_outputs.retain(|id| *id != removed_id);
                    if let Err(e) = tx
                        .unbounded_send(AppEvent::Display(DisplayEvent::RemoveDisplay(removed_id)))
                    {
                        eprintln!("Failed to send remove display event: {}", e);
                    }
                }
            }
        }
    });

    Box::pin(rx)
}
