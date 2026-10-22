use itertools::Itertools;
use typemap_rev::TypeMap;

use crate::data::{HyprMonitors, Monitor, MonitorDescription};

pub fn get(data: &mut TypeMap) -> Option<&mut HyprMonitors> {
    data.get_mut::<Monitor>()
}

pub fn added(
    data: &mut TypeMap,
    id: u32,
    name: String,
    dimensions: (u32, u32),
    position: (i32, i32),
    refresh_rate: f32,
    make: String,
    model: String,
) {
    let new_monitor = Monitor {
        id,
        name: name.clone(),
        dimensions,
        position,
        refresh_rate,
        description: MonitorDescription { make, model },
    };

    if let Some(monitors) = data.get_mut::<Monitor>() {
        monitors.insert(name, new_monitor);
    } else {
        let mut monitors = HyprMonitors::new();
        monitors.insert(name, new_monitor);
        data.insert::<Monitor>(monitors);
    }
}

pub fn removed(monitors: &mut HyprMonitors, id: u32) {
    if let Some(key) = (monitors.iter())
        .find_or_first(|(_, m)| m.id == id)
        .map(|(k, _)| k.clone())
    {
        monitors.remove(&key);
    }
}
