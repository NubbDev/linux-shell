#[derive(Debug, Clone)]
pub enum WidgetEvent {
    Enable { display: String },
    Disable { display: String },
    Section(WidgetType),
}

#[derive(Debug, Clone)]
pub enum WidgetType {
    StatusBar { display: String },
    Dock { display: String },
    SideBar { display: String },
}

impl From<WidgetEvent> for super::AppEvent {
    fn from(event: WidgetEvent) -> Self {
        super::AppEvent::Widgets(event)
    }
}

impl From<WidgetType> for WidgetEvent {
    fn from(widget: WidgetType) -> Self {
        WidgetEvent::Section(widget)
    }
}
