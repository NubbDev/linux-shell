use iced::{Task, alignment::Horizontal, window::Id};
use iced_layershell::reexport::{Anchor, NewLayerShellSettings};
use wayland_client::protocol::wl_output::WlOutput;

use crate::{
    APP_DOCK_DISPLAYS, SIDE_BAR_DISPLAYS, STATUS_BAR_DISPLAYS,
    app::AppState,
    event::{AppEvent, RegisterIdentifier, WidgetEvent, WidgetType, WindowEvent},
    view::{Identifiers, LayerInfo, LayerType, WindowComponent},
};

pub mod sections;

pub fn handle(state: &mut AppState, event: WidgetEvent) -> Task<AppEvent> {
    match event {
        WidgetEvent::Disable { display } => stop_widgets(state, display),
        WidgetEvent::Enable { display } => start_widgets(state, display),
        WidgetEvent::Section(section) => sections::handle(state, section),
    }
}

fn stop_widgets(state: &mut AppState, display: String) -> Task<AppEvent> {
    let layers_to_remove: Vec<Id> = (state.layers.values())
        .filter(|layer| *layer.display() == display)
        .map(|layer| layer.id())
        .collect();

    let windows_to_remove: Vec<Id> = (state.windows.keys())
        .filter(|window_id| {
            (layers_to_remove.iter())
                .any(|layer_id| window_id.to_string().ends_with(&layer_id.to_string()))
        })
        .cloned()
        .collect();

    for window_id in windows_to_remove {
        state.windows.remove(&window_id);
    }

    let tasks = (layers_to_remove.iter())
        .map(|layer_id| Task::done(AppEvent::RemoveWindow(layer_id.clone())))
        .collect::<Vec<_>>();

    for layer_id in layers_to_remove {
        state.layers.remove(&layer_id);
    }

    Task::batch(tasks)
}

fn start_widgets(state: &mut AppState, display: String) -> Task<AppEvent> {
    let mut tasks = vec![];

    if STATUS_BAR_DISPLAYS.contains(&display.as_str()) {
        let display = display.clone();
        let event: WidgetEvent = WidgetType::StatusBar { display }.into();
        tasks.push(Task::done(event.into()));
    }

    if APP_DOCK_DISPLAYS.contains(&display.as_str()) {
        let display = display.clone();
        let event: WidgetEvent = WidgetType::Dock { display }.into();
        tasks.push(Task::done(event.into()));
    }

    if SIDE_BAR_DISPLAYS.contains(&display.as_str()) {
        let display = display.clone();
        let event: WidgetEvent = WidgetType::SideBar { display }.into();
        tasks.push(Task::done(event.into()));
    }

    return Task::batch(tasks);
}
