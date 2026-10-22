use iced::{Padding, Task, alignment::Vertical};

const SPACING: u32 = 6;

use crate::{
    app::AppState,
    data::Monitor,
    event::AppEvent,
    view::{LayerInfo, LayerType},
};

pub fn computate(state: &mut AppState, display: String, align: Vertical) -> Task<AppEvent> {
    let Some(display) = state.get_monitor(&display) else {
        return Task::none();
    };

    let layers = &(state.layers);
    let mut widgets = (layers.values())
        .filter_map(|l| match l.layer_type() {
            LayerType::VerticalWidget { group, position }
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
            Vertical::Top => top(widgets, widget_p, p_size),
            Vertical::Bottom => bottom(widgets, widget_p, p_size),
            Vertical::Center => center(widgets, widget_p, p_size),
        };
        return Task::batch(tasks);
    };

    return Task::none();
}

/*
 * Widget layout functions for vertical alignment (top, bottom, center).
 * Each function calculates the appropriate margins for the widgets based on their sizes,
 * the parent container size, and the specified padding. The resulting margin changes are
 * returned as a vector of tasks that can be executed to update the layout.
 */

fn top(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let mut y_offset = SPACING;

    for (_, layer) in widgets {
        let size = layer.size();
        let center = (parent.0 - size.0) as i32 / 2;

        let top = y_offset as i32 + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = center as i32 + padding.left as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));

        y_offset += size.1 + SPACING;
    }

    tasks
}

fn bottom(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let mut y_offset = SPACING;

    for (_, layer) in widgets.iter().rev() {
        let size = layer.size();
        let center = (parent.0 - size.0) as i32 / 2;

        let left = center + padding.left as i32;
        let bottom = y_offset as i32 + padding.bottom as i32;
        let top = padding.top as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));

        y_offset += size.1 + SPACING;
    }

    tasks
}

fn center(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let total_widget_height: u32 = widgets.iter().map(|(_, l)| l.size().1).sum();
    let total_spacing = (widgets.len().saturating_sub(1) as u32) * SPACING;
    let total_group_height = total_widget_height + total_spacing;

    let mut y_offset = (parent.1 - total_group_height) / 2;
    // let mut x_offset = (parent.0 - total_group_width) / 2;

    for (_, layer) in widgets {
        let size = layer.size();
        let center = (parent.0 as i32 - size.0 as i32) / 2;

        let left = center as i32 + padding.left as i32;
        let bottom = padding.bottom as i32;
        let top = y_offset as i32 + padding.top as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));
        y_offset += size.1 + SPACING;
    }

    tasks
}
