use std::collections::BTreeMap;

use iced::{Color, Font, Task, Theme, theme::Style, window::Id};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use typemap_rev::TypeMap;
use wayland_client::Connection;

use crate::{animations, backend::ImageCache};

use super::{
    animations::Animationable,
    backend::DisplayOutput,
    colors,
    event::AppEvent,
    view::{LayerInfo, WindowComponent},
};

pub struct Connections {
    pub wayland: Connection,
}

impl Default for Connections {
    fn default() -> Self {
        let wayland = Connection::connect_to_env().expect("Failed to connect to Wayland server");
        Self { wayland }
    }
}

pub struct AppState {
    pub displays: BTreeMap<u32, DisplayOutput>,
    pub layers: BTreeMap<Id, LayerInfo>,

    pub windows: BTreeMap<Id, Box<dyn WindowComponent>>,
    pub data: TypeMap,
    pub images: ImageCache,
    pub animations: Vec<Box<dyn Animationable>>,
    pub connections: Connections,
    pub content_generation: u64,
}

impl AppState {
    pub fn get_display_by_name(&self, name: &str) -> Option<&DisplayOutput> {
        self.displays.values().find(|d| d.name() == name)
    }

    pub fn animations<T: Animationable + 'static>(&self) -> Option<&T> {
        self.animations
            .iter()
            .find_map(|a| a.as_any().downcast_ref::<T>())
    }

    pub fn animations_mut<T: Animationable + 'static>(&mut self) -> Option<&mut T> {
        self.animations
            .iter_mut()
            .find_map(|a| a.as_any_mut().downcast_mut::<T>())
    }
}

pub fn run() -> (AppState, Task<AppEvent>) {
    let state = AppState {
        displays: BTreeMap::new(),
        layers: BTreeMap::new(),
        windows: BTreeMap::new(),
        data: TypeMap::new(),
        images: ImageCache::default(),
        animations: animations::setup_animations(),
        connections: Connections::default(),
        content_generation: 0,
    };

    println!("Application started, sending startup event");
    (state, Task::done(AppEvent::Startup))
}

pub fn initial_settings() -> Settings {
    Settings {
        layer_settings: LayerShellSettings {
            start_mode: StartMode::Background,
            ..Default::default()
        },
        antialiasing: true,

        default_font: crate::DEFAULT_FONT,
        ..Default::default()
    }
}

pub fn style(_: &AppState, _: &Theme) -> Style {
    Style {
        background_color: Color::TRANSPARENT,
        text_color: colors::TEXT,
    }
}
