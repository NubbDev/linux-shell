use std::collections::BTreeMap;

use iced::{Task, window::Id};

use crate::{
    animations::marquee::{MarqueeKey, Marquees},
    backend::DisplayOutput,
    event::{AnimEvent, AppEvent, WindowEvent},
    view::{
        LayerInfo, RowGroup, WindowComponent,
        components::marquee_content_id,
        operations::{Bounds, FindBounds},
    },
};

pub fn window_bound(
    windows: &mut BTreeMap<Id, Box<dyn WindowComponent>>,
    layers: &BTreeMap<Id, LayerInfo>,
) -> Task<AppEvent> {
    let mut tasks = Vec::with_capacity(windows.len());

    for (win_id, window) in windows.iter() {
        let Some(_layer) = layers.get(win_id) else {
            continue;
        };

        let id = window.id(*win_id);
        let win_id = *win_id;

        let rect = iced::advanced::widget::operate(FindBounds::new(id))
            .map(move |b| handle_window_bounds(win_id, b))
            .map(AppEvent::Window);
        tasks.push(rect);
    }

    Task::batch(tasks)
}

pub fn display(displays: &BTreeMap<u32, DisplayOutput>) -> Task<AppEvent> {
    let mut tasks = Vec::with_capacity(displays.len());

    for display in displays.values() {
        for group in [RowGroup::Left, RowGroup::Center, RowGroup::Right] {
            let layout = WindowEvent::ComputateLayout(display.clone(), group);
            tasks.push(Task::done(layout).map(AppEvent::Window));
        }
    }

    Task::batch(tasks)
}

pub fn marquee_bounds(map: Option<&Marquees>) -> Task<AppEvent> {
    let mut tasks = Vec::with_capacity(0);

    if let Some(marquees) = map {
        tasks = Vec::with_capacity(marquees.len());
        for key in marquees.keys().into_iter().cloned() {
            let id = marquee_content_id(&key);
            let bounds = iced::advanced::widget::operate(FindBounds::new(id))
                .map(move |b| handle_marquee_bounds(&key, b))
                .map(AppEvent::Animation);
            tasks.push(bounds);
        }
    }

    Task::batch(tasks)
}

fn handle_window_bounds(id: Id, bound: Bounds) -> WindowEvent {
    match bound {
        Bounds::Container(rect) => WindowEvent::BoundsChanged(id, rect),
        _ => WindowEvent::None,
    }
}

fn handle_marquee_bounds(key: &MarqueeKey, bound: Bounds) -> AnimEvent {
    match bound {
        Bounds::Scrollable(outer_rect, inner_rect) => {
            AnimEvent::MarqueeBounds(key.clone(), outer_rect, inner_rect)
        }
        _ => AnimEvent::None,
    }
}
