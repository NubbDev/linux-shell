use iced::{Task, alignment::Horizontal, window::Id};
use iced_layershell::reexport::Anchor;

use super::new_widget;
use crate::{
    app::AppState,
    data::{Displays, Monitor},
    event::AppEvent,
    view::{
        DatePill, LayerType, SpotifyPill, SpotifyProgress, SystemPill, TimePill, VolumePill,
        WorkspacePill,
    },
};

const LEFT_MODULES: &[super::WidgetConstructor] = &[new_widget::<SpotifyPill>];

const CENTER_MODULES: &[super::WidgetConstructor] =
    &[new_widget::<TimePill>, new_widget::<DatePill>];

const RIGHT_MODULES: &[super::WidgetConstructor] = &[
    // new_widget::<VolumePill>,
    new_widget::<SystemPill>,
    new_widget::<SpotifyProgress>,
];

pub fn start(state: &mut AppState, display_name: String) -> Task<AppEvent> {
    let mut tasks = Vec::new();
    if let Some(displays) = state.data.get::<Displays>()
        && let Some(monitors) = state.data.get::<Monitor>()
        && let Some(output) = displays.get(&display_name)
        && let Some(monitor) = monitors.get(&display_name)
    {
        let parent_layer = {
            let layer = super::create_backdrop_layer(
                Anchor::Top | Anchor::Left | Anchor::Right,
                (monitor.dimensions.0, 36),
                (display_name.clone(), output.clone()),
                &mut tasks,
            );
            let id = layer.id();
            state.layers.insert(id, layer);
            id
        };

        for (group, modules) in [
            (Horizontal::Left, LEFT_MODULES),
            (Horizontal::Center, CENTER_MODULES),
            (Horizontal::Right, RIGHT_MODULES),
        ] {
            for (order, constructor) in modules.iter().enumerate() {
                let anchor = match group {
                    Horizontal::Left | Horizontal::Center => Anchor::Left,
                    Horizontal::Right => Anchor::Right,
                };
                let layer_type = LayerType::HorizontalWidget {
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
