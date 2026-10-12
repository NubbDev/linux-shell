use crate::{app::AppState, backend::Patch, event::AppEvent};
use iced::{Task, exit};

mod animation;
mod display;
mod image;
// mod niri;
mod events;
mod register_iden;
mod tick;
mod widget;
mod window;

pub fn run(state: &mut AppState, event: AppEvent) -> Task<AppEvent> {
    // println!("Event: {:?}", event);
    match event {
        AppEvent::Startup => handle_startup(state),
        AppEvent::Exit => handle_exit(state),
        AppEvent::Image(image_event) => image::handle(state, image_event),
        AppEvent::Tick => tick::handle(state),
        AppEvent::AnimationTick => tick::animate(state),
        AppEvent::UpdateData(patch) => update_data(state, patch),
        AppEvent::Window(window_event) => window::handle(state, window_event),
        AppEvent::Display(display_event) => display::handle(state, display_event),
        AppEvent::Widgets(widget_event) => widget::handle(state, widget_event),
        AppEvent::Niri(niri_event) => events::niri::handle(state, niri_event),
        AppEvent::Hyprland(hyprland_event) => events::hyprland::handle(state, hyprland_event),
        AppEvent::Register(iden) => register_iden::handle(state, iden),
        AppEvent::Animation(anim) => animation::handle(state, anim),
        AppEvent::Media(media) => events::media::handle(state, media),

        _ => Task::none(),
    }
}

fn handle_startup(_: &mut AppState) -> Task<AppEvent> {
    Task::none() // This is replace by the actual startup logic you want to implement after all displays are ready.
}

fn handle_exit(_: &mut AppState) -> Task<AppEvent> {
    exit()
}

fn update_data(state: &mut AppState, patch: Box<dyn Patch>) -> Task<AppEvent> {
    state.content_generation += 1;
    patch.apply(&mut state.data)
}
