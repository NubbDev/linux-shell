use iced::{Task, alignment::Horizontal, window::Id};
use iced_layershell::reexport::{Anchor, NewLayerShellSettings};
use wayland_client::protocol::wl_output::WlOutput;

use crate::{
    APP_DOCK_DISPLAYS, STATUS_BAR_DISPLAYS,
    app::AppState,
    event::{AppEvent, RegisterIdentifier, WidgetEvent, WidgetType, WindowEvent},
    view::{Identifiers, LayerInfo, LayerType, WindowComponent},
};

mod dock;
mod sidebar;
mod statusbar;

type WidgetConstructor =
    fn(Anchor, LayerType, Id, String, &WlOutput) -> (LayerInfo, Vec<Task<AppEvent>>);

pub fn handle(state: &mut AppState, section: WidgetType) -> Task<AppEvent> {
    match section {
        WidgetType::StatusBar { display } => statusbar::start(state, display),
        WidgetType::Dock { display } => dock::start(state, display),
        WidgetType::SideBar { display } => sidebar::start(state, display),
    }
}

fn new_widget<T: WindowComponent>(
    anchor: Anchor,
    layer_type: LayerType,
    parent: Id,
    display_name: String,
    output: &WlOutput,
) -> (LayerInfo, Vec<Task<AppEvent>>) {
    let widget = T::new(display_name.clone());
    let anchor = anchor | widget.anchor();

    let namespace = format!("shell-{}", widget.namespace());
    let (layer, settings) =
        new_widget_layer(&namespace, layer_type, display_name, output, anchor, parent);
    let id = layer.id();
    let identifiers = widget.identifiers();
    let tasks = vec![
        Task::done(AppEvent::NewLayerShell { settings, id }),
        Task::done(AppEvent::Window(WindowEvent::open(id, widget))),
        handle_identifiers(id, identifiers),
    ];

    (layer, tasks)
}

pub(super) fn new_widget_layer(
    name: &str,
    layer_type: LayerType,
    display_name: String,
    output: &WlOutput,
    anchor: Anchor,
    parent: Id,
) -> (LayerInfo, NewLayerShellSettings) {
    let id = Id::unique();
    let layer = LayerInfo::new(id, (1, 1), layer_type, anchor, Some(parent), display_name);
    let settings = layer.new_layer(name, output);

    (layer, settings)
}

fn handle_identifiers(id: Id, list: Vec<Identifiers>) -> Task<AppEvent> {
    let tasks = list
        .into_iter()
        .map(|iden| {
            let iden = match iden {
                Identifiers::Marquee(n) => RegisterIdentifier::marquee(id, &n),
            };
            Task::done(iden).map(AppEvent::Register)
        })
        .collect::<Vec<_>>();

    Task::batch(tasks)
}
fn create_backdrop_layer(
    anchor: Anchor,
    size: (u32, u32),
    display: (String, WlOutput),
    tasks: &mut Vec<Task<AppEvent>>,
) -> LayerInfo {
    let layer_type = LayerType::Backdrop;
    let id = Id::unique();
    let layer = LayerInfo::new(id, size, layer_type, anchor, None, display.0);
    tasks.push(Task::done(AppEvent::NewLayerShell {
        settings: layer.empty_layer("shell", display.1),
        id: layer.id(),
    }));

    layer
}
