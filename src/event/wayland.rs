use wayland_client::protocol::wl_output::WlOutput;

#[derive(Debug, Clone)]
pub enum WaylandEvent {
    NewDisplay { name: String, display: WlOutput },
    RemoveDisplay { name: String },
}

impl From<WaylandEvent> for super::AppEvent {
    fn from(event: WaylandEvent) -> Self {
        super::AppEvent::Wayland(event)
    }
}
