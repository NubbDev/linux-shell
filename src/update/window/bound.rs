use iced::{Rectangle, Task, window::Id};

const MARGIN: u32 = 1; // buffer kept above true content size once settled
const GROWTH_STEP: u32 = 5; // how much to grow per probe when clamped

use crate::{app::AppState, event::AppEvent};

pub fn computate(state: &mut AppState, id: Id, bounds: Rectangle) -> Task<AppEvent> {
    let measured = (bounds.width.ceil() as u32, bounds.height.ceil() as u32);
    let current_gen = state.content_generation;
    let Some(layer) = state.layers.get_mut(&id) else {
        return Task::none();
    };
    let applied = layer.size();

    let width_clamped = measured.0 > applied.0;
    let height_clamped = measured.1 > applied.1;

    let anchor = layer.anchor();

    if (width_clamped || height_clamped) && layer.settled_at_generation != current_gen {
        layer.probing = true;
        // only trust a "might be clamped" signal if something actually changed
        // since we last settled — grow only the axis that's actually pinned
        let size = match (width_clamped, height_clamped) {
            (true, true) => (applied.0 + GROWTH_STEP, applied.1 + GROWTH_STEP),
            (true, false) => (applied.0 + GROWTH_STEP, applied.1),
            (false, true) => (applied.0, applied.1 + GROWTH_STEP),
            (false, false) => (applied.0, applied.1),
        };

        layer.set_size(size);
        return Task::done(AppEvent::AnchorSizeChange { id, anchor, size });
    }

    // either genuinely unclamped, or clamped-looking but nothing changed
    // since last settle (so it's the real, stable size) — apply exactly
    layer.settled_at_generation = current_gen;
    layer.probing = false;

    let size = (measured.0 + MARGIN, measured.1 + MARGIN); // tiny 1px pad, your call re: even that
    if size != applied {
        layer.set_size(size);
        return Task::done(AppEvent::AnchorSizeChange { id, anchor, size });
    }

    Task::none()
}
