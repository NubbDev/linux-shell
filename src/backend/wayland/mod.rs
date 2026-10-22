mod output;
use std::{hash::Hash, thread};

use iced::{
    Subscription,
    futures::{
        SinkExt, StreamExt,
        channel::mpsc::{Sender, UnboundedSender, unbounded},
        stream::BoxStream,
    },
};
use wayland_client::Connection;

use crate::event::{AppEvent, WaylandEvent};

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
    let (tx, rx) = unbounded::<AppEvent>();

    println!("Connected to Wayland server");
    thread::spawn(move || {
        let display = connection.display();
        let mut event_queue = connection.new_event_queue::<output::OutputTracker>();
        let qh = event_queue.handle();
        let _registry = display.get_registry(&qh, ());
        let mut output_tracker = output::OutputTracker::default();
        let mut known_outputs: Vec<u32> = Vec::new();
        let mut already_sent_outputs: Vec<u32> = Vec::new();

        loop {
            if event_queue.blocking_dispatch(&mut output_tracker).is_err() {
                break;
            }

            for output in output_tracker.0.iter() {
                let output_id = output.0;
                if !known_outputs.contains(&output_id) {
                    known_outputs.push(output_id);
                } else {
                    if output.1.is_empty() {
                        continue;
                    }
                    if already_sent_outputs.contains(&output_id) {
                        continue;
                    }
                    let event = WaylandEvent::NewDisplay {
                        name: output.1.clone(),
                        display: output.2.clone(),
                    };
                    send_event(tx.clone(), event);
                    already_sent_outputs.push(output_id);
                }
            }

            let removed_outputs: Vec<u32> = known_outputs
                .iter()
                .filter(|id| !output_tracker.0.iter().any(|o| o.0 == **id))
                .copied()
                .collect();

            // get the name of the removed outputs and send a RemoveDisplay event for each
            for removed_output in removed_outputs.iter() {
                if let Some(output) = output_tracker.0.iter().find(|o| o.0 == *removed_output) {
                    let event = WaylandEvent::RemoveDisplay {
                        name: output.1.clone(),
                    };
                    send_event(tx.clone(), event);
                }
                known_outputs.retain(|id| id != removed_output);
                already_sent_outputs.retain(|id| id != removed_output);
            }
        }
    });

    Box::pin(rx)
}

fn send_event(tx: UnboundedSender<AppEvent>, event: WaylandEvent) {
    if let Err(e) = tx.unbounded_send(event.into()) {
        eprintln!("Failed to send Wayland event: {}", e);
    }
}
