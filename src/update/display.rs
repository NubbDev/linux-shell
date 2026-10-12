use iced::{Task, window::Id};
use iced_layershell::reexport::Anchor;

use crate::{
    APP_DOCK_DISPLAYS, STATUS_BAR_DISPLAYS,
    app::AppState,
    backend::DisplayOutput,
    event::{AppEvent, DisplayEvent, WidgetEvent},
    view::{LayerInfo, LayerType},
};

pub fn handle(state: &mut AppState, event: DisplayEvent) -> Task<AppEvent> {
    let display_id = match event {
        DisplayEvent::NewDisplay(display) => {
            state.displays.insert(display.id(), display.clone());
            display.id()
        }
        DisplayEvent::UpdateDisplay(display) => {
            if let Some(existing_display) = state.displays.get_mut(&display.id()) {
                *existing_display = display.clone();
                display.id()
            } else {
                state.displays.insert(display.id(), display.clone());
                display.id()
            }
        }
        DisplayEvent::RemoveDisplay(id) => {
            state.displays.remove(&id);
            return Task::done(AppEvent::Widgets(WidgetEvent::Stop(id)))
        }
    };
    let mut tasks = vec![];
    if let Some(display) = state.displays.get_mut(&display_id)
        && display.is_available()
    {
        println!("Starting Status Bar on display: {}", display.name());
        let anchor = Anchor::Left | Anchor::Right;
        let display_width = display.size().0 as u32;
        if STATUS_BAR_DISPLAYS.contains(&display.name()) {
            let layer = create_backdrop_layer(
                Anchor::Top | anchor,
                (display_width, 30),
                display,
                &mut tasks,
            );
            let layer_id = layer.id();
            state.layers.insert(layer_id, layer);
            tasks.push(
                Task::done(WidgetEvent::StartStatusBar(layer_id, display_id))
                    .map(AppEvent::Widgets),
            );
        }

        if APP_DOCK_DISPLAYS.contains(&display.name()) {
            println!("Starting App Dock on display: {}", display.name());
            let layer = create_backdrop_layer(
                Anchor::Bottom | anchor,
                (display_width, 60),
                display,
                &mut tasks,
            );

            state.layers.insert(layer.id(), layer);
        }
    }
    Task::batch(tasks)
}

fn create_backdrop_layer(
    anchor: Anchor,
    size: (u32, u32),
    display: &DisplayOutput,
    tasks: &mut Vec<Task<AppEvent>>,
) -> LayerInfo {
    let layer_type = LayerType::Backdrop;
    let id = Id::unique();
    let layer = LayerInfo::new(id, size, layer_type, anchor, None, display.id());
    tasks.push(Task::done(AppEvent::NewLayerShell {
        settings: layer.empty_layer("shell", display.proxy().clone()),
        id: layer.id(),
    }));

    layer
}
