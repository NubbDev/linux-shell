use iced::{
    Rectangle,
    widget::{self, image},
    window::Id,
};
use iced_layershell::{actions::LayerShellCustomAction, to_layer_message};

use crate::{
    backend::{DisplayOutput, Patch},
    view::{RowGroup, WindowComponent},
};

mod hyprland;

pub use hyprland::{HyprlandEvent, HyprlandSend, HyprlandReceived};

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum AppEvent {
    Startup,
    Tick,
    AnimationTick,
    Display(DisplayEvent),
    Widgets(WidgetEvent),
    Window(WindowEvent),
    LayerShell(Id, LayerShellCustomAction),
    UpdateData(Box<dyn Patch>),
    Niri(NiriEvent),
    Hyprland(hyprland::HyprlandEvent),
    Image(ImageEvent),
    Register(RegisterIdentifier),
    Animation(AnimEvent),
    Media(MediaEvent),
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
    Open(Id, Box<dyn WindowComponent>),
    ComputateLayout(DisplayOutput, RowGroup),
    BoundsChanged(Id, Rectangle),
    Close(Id),
    None,
}

impl WindowEvent {
    pub fn open<T: WindowComponent>(id: Id, info: T) -> Self {
        WindowEvent::Open(id, Box::new(info))
    }

    pub fn bounds_changed(id: &Id, bounds: Rectangle) -> Self {
        WindowEvent::BoundsChanged(*id, bounds)
    }

    pub fn close(id: Id) -> Self {
        WindowEvent::Close(id)
    }
}

#[derive(Debug, Clone)]
pub enum DisplayEvent {
    NewDisplay(DisplayOutput),
    UpdateDisplay(DisplayOutput),
    RemoveDisplay(u32),
}

#[derive(Debug, Clone)]
pub enum WidgetEvent {
    StartStatusBar(Id, u32),

    Stop(u32),
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
    pub fn marquee(id: Id, name: &str) -> Self {
        RegisterIdentifier::Marquee(crate::animations::marquee::MarqueeKey(id, name.into()))
    }
}

#[derive(Debug, Clone)]
pub enum AnimEvent {
    MarqueeBounds(crate::animations::marquee::MarqueeKey, Rectangle, Rectangle),
    Hover(widget::Id, bool),
    None,
}

#[derive(Debug, Clone)]
pub enum MediaEvent {
    PlayPause(String),
    Next(String),
    Previous(String),
    Stop(String),
    None,
}

impl MediaEvent {
    pub fn get_name(&self) -> Option<&String> {
        match self {
            MediaEvent::PlayPause(name) => Some(name),
            MediaEvent::Next(name) => Some(name),
            MediaEvent::Previous(name) => Some(name),
            MediaEvent::Stop(name) => Some(name),
            MediaEvent::None => None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum TrayEvent {
    Added(String, String),
    Removed(String),
}
