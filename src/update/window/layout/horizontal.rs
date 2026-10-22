use iced::{Padding, Task, alignment::Horizontal};

const SPACING: u32 = 4;

use crate::{
    app::AppState,
    data::Monitor,
    event::AppEvent,
    view::{LayerInfo, LayerType},
};

pub fn computate(state: &mut AppState, display: String, align: Horizontal) -> Task<AppEvent> {
    let Some(display) = state.get_monitor(&display) else {
        return Task::none();
    };

    let layers = &(state.layers);
    let mut widgets = (layers.values())
        .filter_map(|l| match l.layer_type() {
            LayerType::HorizontalWidget { group, position }
                if group == align && display.name == *l.display() && l.size() != (1000, 1000) =>
            {
                Some((position, l))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    widgets.sort_by_key(|(i, _)| *i);

    if let Some(widget) = widgets.first()
        && let Some(p_id) = widget.1.parent()
        && let Some(p) = layers.get(&p_id)
    {
        let p_size = p.size();
        let widget_p = Padding::default();

        let tasks = match align {
            Horizontal::Left => left(widgets, widget_p, p_size),
            Horizontal::Right => right(widgets, widget_p, p_size),
            Horizontal::Center => center(widgets, widget_p, p_size),
        };
        return Task::batch(tasks);
    };

    return Task::none();
}

/*
 * Widget layout functions for horizontal alignment (left, right, center).
 * Each function calculates the appropriate margins for the widgets based on their sizes,
 * the parent container size, and the specified padding. The resulting margin changes are
 * returned as a vector of tasks that can be executed to update the layout.
 */

pub fn left(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let mut x_offset = SPACING;

    for (_, layer) in widgets {
        let size = layer.size();
        let center = (parent.1 as i32 - size.1 as i32) / 2;

        let top = center + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = x_offset as i32 + padding.left as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));

        x_offset += size.0 + padding.left as u32 + SPACING;
    }

    tasks
}

pub fn right(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let mut x_offset = SPACING;

    for (_, layer) in widgets.iter().rev() {
        let size = layer.size();
        let center = (parent.1 as i32 - size.1 as i32) / 2;

        let top = center + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = padding.left as i32;
        let right = x_offset as i32 + padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));

        x_offset += size.0 + padding.right as u32 + SPACING;
    }
    tasks
}

pub fn center(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let total_widget_width: u32 = widgets.iter().map(|(_, l)| l.size().0).sum();
    let total_spacing = (widgets.len().saturating_sub(1) as u32) * SPACING;
    let total_group_width = total_widget_width + total_spacing;

    let mut x_offset = (parent.0 - total_group_width) / 2;

    for (_, layer) in widgets {
        let size = layer.size();
        let center = (parent.1 as i32 - size.1 as i32) / 2;

        let top = center + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = x_offset as i32 + padding.left as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));

        // Advance left offset for the next widget
        x_offset += size.0 + SPACING;
    }
    tasks
}
