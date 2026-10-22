use crate::event::ImageEvent;
use crate::{app::AppState, event::AppEvent};
use dyn_clone::DynClone;
use iced::futures::channel::mpsc::{SendError, Sender, UnboundedSender, channel, unbounded};
use iced::futures::{SinkExt, StreamExt};
use iced::{Subscription, Task};
use iced_runtime::futures::BoxStream;
use iced_runtime::futures::backend::default::time;
use std::fmt::Debug;
use std::time::Duration;
use typemap_rev::TypeMap;

mod app_tray;
mod date_time;
mod hyprland;
mod image;
mod ipc;
mod media;
mod niri;
mod system;
mod volume;
mod wayland;

pub use date_time::TimeDate;
pub use image::{ImageCache, ImageSource};
pub use media::{Spotify, SpotifyCoverArt, SpotifyStatus};
pub use system::{Cpu, Gpu, Memory};
pub use volume::AudioDevices;

pub fn subscription(state: &AppState) -> Subscription<AppEvent> {
    Subscription::batch(vec![
        Subscription::run(process),
        wayland::subscription(state.connections.wayland.clone()),
        time::every(Duration::from_millis(4)).map(|_| AppEvent::Tick),
        time::every(Duration::from_millis(33)).map(|_| AppEvent::AnimationTick),
    ])
}

pub trait Patch: DynClone + Debug + Send + 'static {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent>;
}

dyn_clone::clone_trait_object!(Patch);

#[derive(Debug, Clone)]
enum EventType {
    Event(AppEvent),
    Data(Box<dyn Patch>),
    Image(Box<dyn ImageSource>),
}

impl EventType {
    pub fn event(event: impl Into<AppEvent>) -> Self {
        EventType::Event(event.into())
    }

    pub fn data<T: Patch>(patch: T) -> Self {
        EventType::Data(Box::new(patch))
    }

    pub fn image<T: ImageSource>(source: T) -> Self {
        EventType::Image(Box::new(source))
    }
}

impl From<EventType> for AppEvent {
    fn from(event: EventType) -> Self {
        match event {
            EventType::Event(event) => event,
            EventType::Data(patch) => AppEvent::UpdateData(patch),
            EventType::Image(source) => AppEvent::Image(ImageEvent::Fetch(source)),
        }
    }
}

fn process() -> BoxStream<AppEvent> {
    let (tx, rx) = channel::<EventType>(1000);
    std::thread::spawn(move || {
        let Ok(rt) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        else {
            eprintln!("Failed to build tokio runtime");
            return;
        };
        let local = tokio::task::LocalSet::new();
        println!("Starting backend thread");
        local.block_on(&rt, async move {
            tokio::task::spawn_local(hyprland::run(tx.clone()));
            tokio::task::spawn_local(system::run(tx.clone()));
            tokio::task::spawn_local(media::run(tx.clone()));
            tokio::task::spawn_local(date_time::run(tx.clone()));
            std::future::pending::<()>().await;
        });
    });

    Box::pin(rx.map(|event| event.into()))
}

#[async_trait::async_trait]
trait LoopBackend {
    async fn run(&mut self, sender: Sender<EventType>);
}
