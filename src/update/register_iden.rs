use iced::Task;

use crate::{
    animations::marquee::Marquees,
    app::AppState,
    event::{AppEvent, RegisterIdentifier},
};

pub fn handle(state: &mut AppState, event: RegisterIdentifier) -> Task<AppEvent> {
    match event {
        RegisterIdentifier::Marquee(key) => {
            if let Some(marquees) = state.animations_mut::<Marquees>() {
                marquees.register(key);
            }
        }
    }
    Task::none()
}
