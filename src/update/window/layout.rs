use iced::{Padding, Task};

const SPACING: u32 = 6;

use crate::{
    app::AppState,
    backend::DisplayOutput,
    event::AppEvent,
    view::{LayerInfo, LayerType, RowGroup},
};

pub fn computate(state: &mut AppState, display: DisplayOutput, group: RowGroup) -> Task<AppEvent> {
    let layers = &(state.layers);
    let mut widgets = layers
        .values()
        .filter_map(|l| match l.layer_type() {
            LayerType::Widget(g, i)
                if g == group && display.id() == l.display_id() && l.size() != (1000, 1000) =>
            {
                Some((i, l))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    widgets.sort_by_key(|(i, _)| *i);

    if let Some(w) = widgets.first()
        && let Some(p_id) = w.1.parent()
        && let Some(p) = layers.get(&p_id)
    {
        let p_size = p.size();
        let widget_p = Padding::default().top(2);

        let tasks = match group {
            RowGroup::Left => handle_left(widgets, widget_p, p_size),
            RowGroup::Right => handle_right(widgets, widget_p, p_size),
            RowGroup::Center => handle_center(widgets, widget_p, p_size),
        };
        return Task::batch(tasks);
    };

    return Task::none();
}

fn handle_left(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let mut offset = SPACING;
    for (_, layer) in widgets {
        let center = (parent.1 - layer.size().1) as i32 / 2;

        let top = center + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = offset as i32 + padding.left as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));
        offset += layer.size().0 + SPACING;
    }
    tasks
}

fn handle_right(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let mut offset = SPACING;
    for (_, layer) in widgets.iter().rev() {
        let center = (parent.1 - layer.size().1) as i32 / 2;

        let top = center + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = padding.left as i32;
        let right = offset as i32 + padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));
        offset += layer.size().0 + SPACING;
    }
    tasks
}

fn handle_center(
    widgets: Vec<(usize, &LayerInfo)>,
    padding: Padding,
    parent: (u32, u32),
) -> Vec<Task<AppEvent>> {
    let mut tasks = Vec::new();
    let total_widget_width: i32 = widgets.iter().map(|(_, l)| l.size().0 as i32).sum();
    let total_spacing = (widgets.len().saturating_sub(1) as i32) * SPACING as i32;
    let total_group_width = total_widget_width + total_spacing;

    let mut offset = (parent.0 as i32 - total_group_width) / 2;

    for (_, layer) in widgets {
        let center = (parent.1 - layer.size().1) as i32 / 2;

        let top = center + padding.top as i32;
        let bottom = padding.bottom as i32;
        let left = offset + padding.left as i32;
        let right = padding.right as i32;

        let event = AppEvent::MarginChange {
            id: layer.id(),
            margin: (top, right, bottom, left),
        };
        tasks.push(Task::done(event));

        // Advance left offset for the next widget
        offset += layer.size().0 as i32 + SPACING as i32;
    }
    tasks
}
