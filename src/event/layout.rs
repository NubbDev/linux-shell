use iced::{
    Rectangle,
    alignment::{Horizontal, Vertical},
    window::Id,
};

#[derive(Debug, Clone)]
pub enum WidgetLayout {
    Horizontal { display: String, align: Horizontal },
    Vertical { display: String, align: Vertical },
    Bounds { window: Id, size: Rectangle },
}

impl From<WidgetLayout> for super::AppEvent {
    fn from(layout: WidgetLayout) -> Self {
        super::AppEvent::Layout(layout)
    }
}
