use iced::{
    Task,
    alignment::{Horizontal, Vertical},
    window::Id,
};
use iced_layershell::reexport::Anchor;

use super::new_widget;
use crate::{
    app::AppState,
    data::{Displays, Monitor},
    event::AppEvent,
    view::{LayerType, WorkspacePill},
};

const TOP_MODULES: &[super::WidgetConstructor] = &[];

const CENTER_MODULES: &[super::WidgetConstructor] = &[super::new_widget::<WorkspacePill>];

const BOTTOM_MODULES: &[super::WidgetConstructor] = &[];

pub fn start(state: &mut AppState, display_name: String) -> Task<AppEvent> {
    let mut tasks = Vec::new();
    if let Some(displays) = state.data.get::<Displays>()
        && let Some(monitors) = state.data.get::<Monitor>()
        && let Some(output) = displays.get(&display_name)
        && let Some(monitor) = monitors.get(&display_name)
    {
        let parent_layer = {
            let layer = super::create_backdrop_layer(
                Anchor::Top | Anchor::Left | Anchor::Bottom,
                (36, monitor.dimensions.1),
                (display_name.clone(), output.clone()),
                &mut tasks,
            );
            let id = layer.id();
            state.layers.insert(id, layer);
            id
        };

        for (group, modules) in [
            (Vertical::Top, TOP_MODULES),
            (Vertical::Center, CENTER_MODULES),
            (Vertical::Bottom, BOTTOM_MODULES),
        ] {
            for (order, constructor) in modules.iter().enumerate() {
                let anchor = match group {
                    Vertical::Top | Vertical::Center => Anchor::Top,
                    Vertical::Bottom => Anchor::Bottom,
                };
                let layer_type = LayerType::VerticalWidget {
                    group,
                    position: order,
                };

                let (layer, task) = constructor(
                    anchor,
                    layer_type,
                    parent_layer,
                    display_name.clone(),
                    &output,
                );
                state.layers.insert(layer.id(), layer);
                tasks.extend(task);
            }
        }
    }

    Task::batch(tasks)
}
