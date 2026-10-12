use wayland_client::{
    Dispatch, Proxy,
    protocol::{
        wl_output::{Mode, Transform, WlOutput},
        wl_registry::WlRegistry,
    },
};

#[derive(Debug, Clone)]
pub struct DisplayOutput {
    proxy: WlOutput,
    id: u32,
    name: String,
    description: String,
    scale: i32,
    transform: Option<Transform>,
    size: (i32, i32),
    ready: bool,
}

impl DisplayOutput {
    pub fn new(id: u32, proxy: WlOutput) -> Self {
        Self {
            proxy,
            id,
            name: String::new(),
            description: String::new(),
            scale: 1,
            transform: None,
            size: (0, 0),
            ready: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    pub fn transform(&self) -> Option<Transform> {
        self.transform
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn proxy(&self) -> &WlOutput {
        &self.proxy
    }

    pub fn is_available(&self) -> bool {
        self.ready
    }
}

#[derive(Debug, Default)]
pub struct OutputTracker(pub Vec<DisplayOutput>);

impl Dispatch<WlRegistry, ()> for OutputTracker {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: wayland_client::protocol::wl_registry::Event,
        _: &(),
        _: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wayland_client::protocol::wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                if interface.as_str() == "wl_output" {
                    let version = version.min(WlOutput::interface().version);
                    let output = proxy.bind::<WlOutput, _, _>(name, version, qh, ());
                    state.0.push(DisplayOutput::new(name, output));
                }
            }
            wayland_client::protocol::wl_registry::Event::GlobalRemove { name } => {
                (state.0.iter().find(|output| output.id == name))
                    .map(|output| output.proxy.release());
                state.0.retain(|output| output.id != name);
            }
            _ => {}
        }
    }
}

impl Dispatch<WlOutput, ()> for OutputTracker {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: wayland_client::protocol::wl_output::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        let Some(output) = state.0.iter_mut().find(|output| output.proxy == *proxy) else {
            println!("Received event for unknown output: {:?}", proxy);
            return;
        };
        match event {
            wayland_client::protocol::wl_output::Event::Name { name } => output.name = name,
            wayland_client::protocol::wl_output::Event::Description { description } => {
                output.description = description
            }
            wayland_client::protocol::wl_output::Event::Scale { factor } => output.scale = factor,
            wayland_client::protocol::wl_output::Event::Geometry { transform, .. } => {
                match transform.into_result() {
                    Ok(transform) => output.transform = Some(transform),
                    Err(_) => println!("Unknown transform value: {:?}", transform),
                }
            }
            wayland_client::protocol::wl_output::Event::Mode {
                flags,
                width,
                height,
                ..
            } => match flags.into_result() {
                Ok(flags) => {
                    if flags.contains(Mode::Current) {
                        output.size = (width, height);
                    }
                }
                Err(_) => println!("Unknown mode flags value: {:?}", flags),
            },
            wayland_client::protocol::wl_output::Event::Done => output.ready = true,
            _ => {}
        }
    }
}
