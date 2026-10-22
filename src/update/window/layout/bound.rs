use crate::{app::AppState, event::AppEvent, view::LayerInfo};
use iced::{Rectangle, Task, window::Id};

pub fn computate(state: &mut AppState, id: Id, bounds: Rectangle) -> Task<AppEvent> {
    let Some(layer) = state.layers.get_mut(&id) else {
        return Task::none();
    };

    let measured = (bounds.width.ceil() as u32, bounds.height.ceil() as u32);

    if layer.size() == measured {
        return Task::none();
    }

    layer.set_size(measured);
    let event = AppEvent::AnchorSizeChange {
        id,
        anchor: layer.anchor(),
        size: measured,
    };
    Task::done(event)
}
