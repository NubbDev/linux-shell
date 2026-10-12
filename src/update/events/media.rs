use iced::Task;
use mpris::PlayerFinder;

use crate::{
    app::AppState,
    event::{AppEvent, MediaEvent},
};
pub fn handle(_: &mut AppState, event: MediaEvent) -> Task<AppEvent> {
    Task::perform(
        async move {
            let finder = PlayerFinder::new().expect("mpris player finder failed");
            if let Some(name) = event.get_name()
                && let Ok(player) = finder.find_by_name(name)
            {
                match event {
                    MediaEvent::PlayPause(_) => {
                        player.play_pause().ok();
                    }
                    MediaEvent::Next(_) => {
                        player.next().ok();
                    }
                    MediaEvent::Previous(_) => {
                        player.previous().ok();
                    }
                    _ => {}
                };
            }
        },
        |_| AppEvent::Empty,
    )
}
