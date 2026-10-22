use std::time::Instant;

use iced::{
    Border, Element, Padding, Theme,
    widget::{Id, MouseArea, column, container, mouse_area, row},
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;
use typemap_rev::TypeMap;

use crate::{
    animations::{AnimationableProgress, workspace::WorkspaceActive},
    app::AppState,
    colors::{self, mix_colors},
    data::{Monitor, Workspace},
    event::{AppEvent, HyprlandEvent, HyprlandSend},
    view::{LayerInfo, components::window},
};

use super::{WindowComponent, pills_style};

const PILL_HEIGHT: f32 = PILL_WIDTH * 2.0;
const PILL_WIDTH: f32 = 21.0;

#[derive(Debug, Clone)]
pub struct WorkspacePill {
    display: String,
}

impl WindowComponent for WorkspacePill {
    fn new(display: String) -> Self {
        Self { display }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        if let Some(layer) = state.layers.get(&id)
            && let Some(animation) = state.animations::<WorkspaceActive>()
        {
            let workspaces = load_workspaces(&state.data, layer);
            let col_box =
                column(workspaces.into_iter().map(|w| pill(w, animation).into())).spacing(2);

            return window(
                container(col_box)
                    .padding(Padding::from([4, 5]))
                    .style(|_| pills_style(colors::COLOR3)),
            )
            .id(self.id(id))
            .into();
        };
        container("").into()
    }

    fn id(&self, id: WindowId) -> Id {
        Id::from(format!("{}-{}", self.namespace(), id))
    }

    fn display(&self) -> &String {
        &self.display
    }

    fn namespace(&self) -> String {
        "pill-workspace".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Left
    }
}

fn pill<'a>(workspace: &'a Workspace, animation: &'a WorkspaceActive) -> MouseArea<'a, AppEvent> {
    let index = workspace.id;
    let now = Instant::now();

    let (height, color) = match animation.progress(&index, now) {
        Some(progress) => (
            PILL_WIDTH + (PILL_HEIGHT - PILL_WIDTH) * progress,
            mix_colors(colors::COLOR4, colors::COLOR3, progress),
        ),
        None => (PILL_WIDTH, colors::COLOR4),
    };

    let click_event: HyprlandEvent = HyprlandSend::SwitchWorkspace(index).into();

    mouse_area(
        container("")
            .height(height)
            .width(PILL_WIDTH)
            .style(move |_: &Theme| container::Style {
                background: Some(color.into()),
                border: Border {
                    radius: 1.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
    )
    .on_press(click_event.into())
}

fn load_workspaces<'a>(data: &'a TypeMap, layer: &'a LayerInfo) -> Vec<&'a Workspace> {
    if let Some(monitors) = data.get::<Monitor>()
        && let Some(monitor) = monitors.get(layer.display())
        && let Some(ws) = data.get::<Workspace>()
    {
        let mut workspaces = (ws.iter().filter(|(_, w)| w.monitor == monitor.name))
            .map(|(_, w)| w)
            .collect::<Vec<_>>();
        workspaces.sort_by_key(|w| w.position);
        return workspaces;
    }
    vec![]
}
