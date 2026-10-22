use wayland_client::{
    Dispatch, Proxy,
    protocol::{
        wl_output::{self, WlOutput},
        wl_registry::{self, WlRegistry},
    },
};

#[derive(Debug, Default)]
pub struct OutputTracker(pub Vec<(u32, String, WlOutput)>);

impl Dispatch<WlRegistry, ()> for OutputTracker {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                if interface.as_str() == "wl_output" {
                    let version = version.min(WlOutput::interface().version);
                    let output = proxy.bind::<WlOutput, _, _>(name, version, qh, ());
                    state.0.push((name, "".into(), output));
                }
            }
            wl_registry::Event::GlobalRemove { name } => {
                (state.0.iter().find(|output| output.0 == name)).map(|output| output.2.release());
                state.0.retain(|output| output.0 != name);
            }
            _ => {}
        }
    }
}

impl Dispatch<WlOutput, ()> for OutputTracker {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let Some(output) = state.0.iter_mut().find(|output| output.2 == *proxy) {
            match event {
                wl_output::Event::Name { name } => output.1 = name,
                _ => {}
            }
        };
    }
}
