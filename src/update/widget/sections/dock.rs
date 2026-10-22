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

pub fn start(state: &mut AppState, display_name: String) -> Task<AppEvent> {
    let mut tasks = Vec::new();
    if let Some(displays) = state.data.get::<Displays>()
        && let Some(monitors) = state.data.get::<Monitor>()
        && let Some(output) = displays.get(&display_name)
        && let Some(monitor) = monitors.get(&display_name)
    {
        let layer = super::create_backdrop_layer(
            Anchor::Bottom | Anchor::Left | Anchor::Right,
            (monitor.dimensions.0, 60),
            (display_name, output.clone()),
            &mut tasks,
        );

        state.layers.insert(layer.id(), layer);
    }

    Task::batch(tasks)
}
