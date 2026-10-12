use iced::{Task, window::Id};

use crate::{
    app::AppState,
    event::{AppEvent},
    view::{
        DatePill,  RowGroup, SpotifyPill, SpotifyProgress,
        SystemPill, TimePill, VolumePill, WorkspacePill,
    },
};
use super::new_widget;

const LEFT_MODULES: &[super::WidgetConstructor] = &[
    new_widget::<WorkspacePill>,
    new_widget::<SpotifyPill>
];

const CENTER_MODULES: &[super::WidgetConstructor] = &[
    new_widget::<SpotifyProgress>,
    new_widget::<TimePill>,
    new_widget::<DatePill>,
];

const RIGHT_MODULES: &[super::WidgetConstructor] = &[
    new_widget::<VolumePill>,
    new_widget::<SystemPill>
];

pub fn start(state: &mut AppState, parent_layer: Id, output: u32) -> Task<AppEvent> {
    let displays = &(state.displays);
    let Some(display) = displays.get(&output) else {
        return Task::none();
    };
    let mut tasks = Vec::new();
    for (group, modules) in [
        (RowGroup::Left, LEFT_MODULES),
        (RowGroup::Center, CENTER_MODULES),
        (RowGroup::Right, RIGHT_MODULES),
    ] {
        for (order, constructor) in modules.iter().enumerate() {
            let (layer, task) = constructor(group, order, parent_layer, display);
            state.layers.insert(layer.id(), layer);
            tasks.extend(task);
        }
    }

    Task::batch(tasks)
}
