use iced::{Task, widget::image::Handle};

use crate::{
    app::AppState,
    backend::ImageSource,
    event::{AppEvent, ImageEvent},
};

pub fn handle(state: &mut AppState, event: ImageEvent) -> Task<AppEvent> {
    match event {
        ImageEvent::Fetch(image) => fetch_image(image),
        ImageEvent::Load(url, handle) => load_image(state, url, handle),
    }
}

fn fetch_image(img: Box<dyn ImageSource>) -> Task<AppEvent> {
    let url = img.url();
    let hash: String = img.hash();
    Task::perform(
        async move {
            let bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
            Some(Handle::from_bytes(bytes))
        },
        |handle| {
            if let Some(handle) = handle {
                return AppEvent::Image(ImageEvent::Load(hash, handle));
            }
            AppEvent::Empty
        },
    )
}

fn load_image(state: &mut AppState, hash: String, handle: Handle) -> Task<AppEvent> {
    state.images.insert(hash, handle);
    Task::none()
}
