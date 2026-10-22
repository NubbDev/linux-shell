use iced::{Rectangle, widget::Id as WidgetId, widget::image, window::Id as WindowId};
use iced_layershell::{actions::LayerShellCustomAction, to_layer_message};

use crate::{backend::Patch, data::Monitor, view::WindowComponent};

mod hyprland;
mod layout;
mod media;
mod wayland;
mod widget;

pub use hyprland::{HyprlandEvent, HyprlandReceived, HyprlandSend};
pub use layout::WidgetLayout;
pub use media::MediaEvent;
pub use wayland::WaylandEvent;
pub use widget::{WidgetEvent, WidgetType};

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum AppEvent {
    Startup,
    Tick,
    AnimationTick,
    Window(WindowEvent),
    Widgets(widget::WidgetEvent),
    Wayland(wayland::WaylandEvent),
    Hyprland(hyprland::HyprlandEvent),
    Layout(layout::WidgetLayout),
    Media(media::MediaEvent),

    LayerShell(WindowId, LayerShellCustomAction),
    UpdateData(Box<dyn Patch>),
    Niri(NiriEvent),
    Image(ImageEvent),
    Register(RegisterIdentifier),
    Animation(AnimEvent),
    AppTray(),
    Empty,
    Exit,
}

impl AppEvent {
    pub fn update_data<T: Patch>(patch: T) -> Self {
        AppEvent::UpdateData(Box::new(patch))
    }
}

#[derive(Debug, Clone)]
pub enum WindowEvent {
    Open(WindowId, Box<dyn WindowComponent>),
    Close(WindowId),
    None,
}

impl WindowEvent {
    pub fn open<T: WindowComponent>(id: WindowId, info: T) -> Self {
        WindowEvent::Open(id, Box::new(info))
    }

    pub fn close(id: WindowId) -> Self {
        WindowEvent::Close(id)
    }
}

#[derive(Debug, Clone)]
pub enum NiriEvent {
    SwitchWorkspace(u64),
}

#[derive(Debug, Clone)]
pub enum ImageEvent {
    Fetch(Box<dyn crate::backend::ImageSource>),
    Load(String, image::Handle),
}

#[derive(Debug, Clone)]
pub enum RegisterIdentifier {
    Marquee(crate::animations::marquee::MarqueeKey),
}

impl RegisterIdentifier {
    pub fn marquee(id: WindowId, name: &str) -> Self {
        RegisterIdentifier::Marquee(crate::animations::marquee::MarqueeKey(id, name.into()))
    }
}

#[derive(Debug, Clone)]
pub enum AnimEvent {
    MarqueeBounds(crate::animations::marquee::MarqueeKey, Rectangle, Rectangle),
    Hover(WidgetId, bool),
    None,
}

#[derive(Debug, Clone)]
pub enum TrayEvent {
    Added(String, String),
    Removed(String),
}
