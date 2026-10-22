use std::collections::HashMap;

use typemap_rev::TypeMapKey;
use wayland_client::protocol::wl_output::WlOutput;

pub struct Displays;

impl TypeMapKey for Displays {
    type Value = HashMap<String, WlOutput>;
}
