use std::collections::BTreeMap;

use iced::{Element, widget::container, window::Id};
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
pub enum RowGroup {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerType {
    Backdrop,
    Widget(RowGroup, usize),
}

#[derive(Debug, Clone)]
pub struct LayerInfo {
    id: Id,
    size: (u32, u32),
    display_id: u32,
    anchor: Anchor,
    parent: Option<Id>,
    layer_type: LayerType,
    pub probing: bool,
    pub settled_at_generation: u64,
}

impl Default for LayerInfo {
    fn default() -> Self {
        Self {
            id: Id::unique(),
            size: (0, 0),
            display_id: 0,
            anchor: Anchor::Top,
            probing: true,
            settled_at_generation: 0,
            parent: None,
            layer_type: LayerType::Backdrop,
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
        display_id: u32,
    ) -> Self {
        Self {
            id,
            size,
            display_id,
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

    pub fn display_id(&self) -> u32 {
        self.display_id
    }

    pub fn layer_type(&self) -> LayerType {
        self.layer_type
    }

    pub fn parent(&self) -> Option<Id> {
        self.parent
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    pub fn set_parent(&mut self, parent: Option<Id>) {
        self.parent = parent;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    pub fn empty_layer(&self, name: &str, output: WlOutput) -> NewLayerShellSettings {
        NewLayerShellSettings {
            exclusive_zone: Some(self.size.1 as i32),
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
