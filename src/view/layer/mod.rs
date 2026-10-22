use std::collections::BTreeMap;

use iced::{
    Element,
    alignment::{Horizontal, Vertical},
    widget::container,
    window::Id,
};
use iced_layershell::reexport::{
    Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings, OutputOption,
};
use wayland_client::protocol::wl_output::WlOutput;

use crate::event::AppEvent;

pub fn handle_backdrop_layers(
    backdrops: &BTreeMap<Id, LayerInfo>,
    id: Id,
) -> Option<Element<'_, AppEvent>> {
    if !backdrops.contains_key(&id) {
        return None;
    }

    Some(container("").into())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerType {
    Backdrop,
    HorizontalWidget { group: Horizontal, position: usize },
    VerticalWidget { group: Vertical, position: usize },
}

#[derive(Debug, Clone)]
pub struct LayerInfo {
    id: Id,
    size: (u32, u32),          // Most recent size sent to layershell
    measured_size: (u32, u32), // Most recent size measured by iced
    display: String,
    anchor: Anchor,
    parent: Option<Id>,
    layer_type: LayerType,
}

impl Default for LayerInfo {
    fn default() -> Self {
        Self {
            id: Id::unique(),
            size: (0, 0),
            display: String::new(),
            anchor: Anchor::Top,
            parent: None,
            layer_type: LayerType::Backdrop,
            measured_size: (0, 0),
        }
    }
}

impl LayerInfo {
    pub fn new(
        id: Id,
        size: (u32, u32),
        layer_type: LayerType,
        anchor: Anchor,
        parent: Option<Id>,
        display: String,
    ) -> Self {
        Self {
            id,
            size,
            display,
            anchor,
            layer_type,
            parent,
            ..Default::default()
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    pub fn display(&self) -> &String {
        &self.display
    }

    pub fn layer_type(&self) -> LayerType {
        self.layer_type
    }

    pub fn parent(&self) -> Option<Id> {
        self.parent
    }

    pub fn set_size(&mut self, size: (impl Into<u32>, impl Into<u32>)) {
        self.size = (size.0.into(), size.1.into());
    }

    pub fn set_target_size(&mut self, size: (impl Into<u32>, impl Into<u32>)) {
        self.measured_size = (size.0.into(), size.1.into());
    }

    pub fn set_parent(&mut self, parent: Option<Id>) {
        self.parent = parent;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    pub fn empty_layer(&self, name: &str, output: WlOutput) -> NewLayerShellSettings {
        if self.size.0 == 0 || self.size.1 == 0 {
            panic!("Layer size is zero, cannot create empty layer");
        }
        let exclusive_zone = if self.size.0 > self.size.1 {
            Some(self.size.1 as i32)
        } else {
            Some(self.size.0 as i32)
        };
        NewLayerShellSettings {
            exclusive_zone,
            anchor: self.anchor,
            events_transparent: true,
            keyboard_interactivity: KeyboardInteractivity::None,
            output_option: OutputOption::Output(output),
            layer: Layer::Top,
            margin: None,
            namespace: name.to_string().into(),
            size: Some(self.size),
        }
    }

    pub fn new_layer(&self, name: &str, output: &WlOutput) -> NewLayerShellSettings {
        NewLayerShellSettings {
            exclusive_zone: Some(-1),
            anchor: self.anchor,
            events_transparent: false,
            keyboard_interactivity: KeyboardInteractivity::None,
            output_option: OutputOption::Output(output.clone()),
            layer: Layer::Top,
            margin: None,
            namespace: name.to_string().into(),
            size: Some(self.size),
        }
    }
}
