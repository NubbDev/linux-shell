use std::collections::BTreeMap;

use typemap_rev::TypeMapKey;

pub type HyprWorkspaces = BTreeMap<i32, Workspace>;
pub type HyprMonitors = BTreeMap<String, Monitor>;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub is_active: bool,
    pub position: i32,
}

impl TypeMapKey for Workspace {
    type Value = HyprWorkspaces;
}

#[derive(Debug, Clone)]
pub struct Monitor {
    // The unique identifier for the monitor that is used by the Hyprland compositor
    pub id: u32,
    // The name of the monitor (e.g., "HDMI-A-1", "DP-1")
    pub name: String,
    // The dimensions of the monitor in pixels (width, height)
    pub dimensions: (u32, u32),
    // The position of the monitor in pixels (x, y)
    pub position: (i32, i32),
    // The refresh rate of the monitor in Hz
    pub refresh_rate: f32,

    pub description: MonitorDescription,
}

#[derive(Debug, Clone)]
pub struct MonitorDescription {
    pub make: String,
    pub model: String,
}

impl TypeMapKey for Monitor {
    type Value = HyprMonitors;
}
