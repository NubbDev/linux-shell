use crate::{
    app::AppState,
    data::Displays,
    event::{AppEvent, WaylandEvent, WidgetEvent},
};

use iced::Task;
use std::{collections::HashMap, time::Duration};
use typemap_rev::{TypeMap, TypeMapKey};
use wayland_client::protocol::wl_output::WlOutput;

struct CachedDisplay;

impl TypeMapKey for CachedDisplay {
    type Value = HashMap<u32, WlOutput>;
}

pub fn handle(state: &mut AppState, event: WaylandEvent) -> Task<AppEvent> {
    match event {
        WaylandEvent::NewDisplay { name, display } => new_display(&mut state.data, name, display),
        WaylandEvent::RemoveDisplay { name } => remove_display(&mut state.data, name),
    }
}

fn new_display(data: &mut TypeMap, name: String, output: WlOutput) -> Task<AppEvent> {
    if let Some(displays) = data.get_mut::<Displays>() {
        displays.insert(name.clone(), output);
    } else {
        let mut new_map = HashMap::new();
        new_map.insert(name.clone(), output);
        data.insert::<Displays>(new_map);
    }
    println!("New display added: {}", name);
    Task::perform(tokio::time::sleep(Duration::from_millis(500)), move |_| {
        WidgetEvent::Enable { display: name }.into()
    })
}

fn remove_display(data: &mut TypeMap, display: String) -> Task<AppEvent> {
    println!("Removing display: {}", display);
    if let Some(displays) = data.get_mut::<Displays>() {
        displays.remove(&display);
        return Task::done(WidgetEvent::Disable { display }.into());
    }
    Task::none()
}
