use iced::{Task, window::Id};
use iced_layershell::reexport::{Anchor, NewLayerShellSettings};

use crate::{
    app::AppState,
    backend::DisplayOutput,
    event::{AppEvent, RegisterIdentifier, WidgetEvent, WindowEvent},
    view::{
        DatePill, Identifiers, LayerInfo, LayerType, RowGroup, SpotifyPill, SpotifyProgress,
        SystemPill, TimePill, VolumePill, WindowComponent, WorkspacePill,
    },
};

mod statusbar;
mod dock;
mod sidebar;

type WidgetConstructor =
    fn(RowGroup, usize, Id, &DisplayOutput) -> (LayerInfo, Vec<Task<AppEvent>>);

pub fn handle(state: &mut AppState, event: WidgetEvent) -> Task<AppEvent> {
    match event {
        WidgetEvent::StartStatusBar(p_layer, output) => statusbar::start(state, p_layer, output),
        WidgetEvent::Stop(output) => {

            let layers_to_remove: Vec<Id> = state
                .layers
                .values()
                .filter(|layer| layer.display_id() == output)
                .map(|layer| layer.id())
                .collect();

            // window ids use layer ids in the id name.

            let windows_to_remove: Vec<Id> = state
                .windows
                .keys()
                .filter(|window_id| {
                    layers_to_remove.iter().any(|layer_id| {
                        window_id.to_string().ends_with(&layer_id.to_string())
                    })
                })
                .cloned()
                .collect();

            for window_id in windows_to_remove {
                state.windows.remove(&window_id);
            }

            // let tasks = layers_to_remove
            //     .iter()
            //     .map(|layer_id| Task::done(AppEvent::Laye))
            //     .collect::<Vec<_>>();
            for layer_id in layers_to_remove {
                state.layers.remove(&layer_id);
            }

            Task::none()
        }
        _ => Task::none(),
    }
}



fn new_widget<T: WindowComponent>(
    group: RowGroup,
    order: usize,
    parent: Id,
    display: &DisplayOutput,
) -> (LayerInfo, Vec<Task<AppEvent>>) {
    let widget = T::new(display.id());
    let layer_type = LayerType::Widget(group, order);
    let anchor = match group {
        RowGroup::Left => Anchor::Left | widget.anchor(),
        RowGroup::Center => Anchor::Left | widget.anchor(),
        RowGroup::Right => Anchor::Right | widget.anchor(),
    };

    let namespace = format!("shell-{}", widget.namespace());
    let (layer, settings) = new_widget_layer(&namespace, layer_type, display, anchor, parent);
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
    display: &DisplayOutput,
    anchor: Anchor,
    parent: Id,
) -> (LayerInfo, NewLayerShellSettings) {
    let id = Id::unique();
    let layer = LayerInfo::new(
        id,
        (1000, 1000),
        layer_type,
        anchor,
        Some(parent),
        display.id(),
    );
    let settings = layer.new_layer(name, display.proxy());

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
