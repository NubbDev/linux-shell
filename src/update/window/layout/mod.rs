use hyprland::event_listener::LayoutEvent;
use iced::{Padding, Task, alignment::Horizontal};

const SPACING: u32 = 6;

use crate::{
    app::AppState,
    data::Monitor,
    event::{AppEvent, WidgetLayout},
    view::{LayerInfo, LayerType},
};

mod bound;
mod horizontal;
mod vertical;

pub fn handle(state: &mut AppState, event: WidgetLayout) -> Task<AppEvent> {
    match event {
        WidgetLayout::Bounds { window, size } => bound::computate(state, window, size),
        WidgetLayout::Vertical { display, align } => vertical::computate(state, display, align),
        WidgetLayout::Horizontal { display, align } => horizontal::computate(state, display, align),
    }
}
