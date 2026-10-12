use crate::{app::AppState, event::AppEvent};
use iced::{Element, widget::text, window::Id};

pub mod components;
pub mod icons;
mod layer;
pub mod operations;
mod windows;

pub use layer::{LayerInfo, LayerType, RowGroup};
pub use windows::*;

pub fn run(state: &AppState, id: Id) -> Element<'_, AppEvent> {
    if !state.windows.contains_key(&id)
        && let Some(element) = layer::handle_backdrop_layers(&state.layers, id)
    {
        return element;
    }

    let Some(window) = state.windows.get(&id) else {
        return text("Window not found").into();
    };

    window.view(id, state)
}
