
use serde::{Deserialize, Deserializer};

use super::workspace;

#[derive(Debug, Clone, Deserialize)]
pub struct RawHyprWindow {
    address: String,
    hidden: bool,
    visible: bool,
    #[serde(rename = "acceptsInput")]
    accepts_input: bool,
    at: [i32; 2],
    size: [u32; 2],
    #[serde(deserialize_with = "workspace::handle_de")]
    workspace: u32,
    floating: bool,
    monitor: u32,
    class: String,
    title: String,
    #[serde(rename = "initialClass")]
    initial_class: String,
    #[serde(rename = "initialTitle")]
    initial_title: String,
    pid: u32,
    xwayland: bool,
    pinned: bool,
    #[serde(rename = "pinFullscreened")]
    pin_fullscreened: bool,
    fullscreen: u32,
    #[serde(rename = "fullscreenClient")]
    fullscreen_client: u32,
    #[serde(rename = "fullscreenHandler")]
    fullscreen_handler: String,
    #[serde(rename = "allowedOverFullscreen")]
    allowed_over_fullscreen: bool,
    #[serde(rename = "contentType")]
    content_type: String,
}
