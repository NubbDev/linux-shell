use iced::Task;
use niri_ipc::{Action, Request, WorkspaceReferenceArg, socket::Socket};

use crate::{
    app::AppState,
    event::{AppEvent, NiriEvent},
};

pub fn handle(_: &mut AppState, event: NiriEvent) -> Task<AppEvent> {
    if let Ok(mut socket) = Socket::connect() {
        return Task::perform(
            async move {
                match event {
                    NiriEvent::SwitchWorkspace(id) => {
                        socket.send(Request::Action(Action::FocusWorkspace {
                            reference: WorkspaceReferenceArg::Id(id),
                        }))
                    }
                }
            },
            |_| AppEvent::Empty,
        );
    }
    Task::none()
}
