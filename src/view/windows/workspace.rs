use std::time::Instant;

use iced::{
    Border, Element, Padding, Theme,
    widget::{Id, MouseArea, container, mouse_area, row},
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;

use crate::{
    animations::{AnimationableProgress, workspace::WorkspaceActive}, app::AppState,  colors::{self, mix_colors}, event::{AppEvent, HyprlandEvent, HyprlandSend}, view::LayerInfo,
};

use super::{WindowComponent, pills_style};

const PILL_HEIGHT: f32 = 21.0;
const PILL_WIDTH: f32 = PILL_HEIGHT * 2.0;

#[derive(Debug, Clone)]
pub struct WorkspacePill {
    display_id: u32,
}

impl WindowComponent for WorkspacePill {
    fn new(display_id: u32) -> Self {
        Self { display_id }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        // if let Some(layer) = state.layers.get(&id)
        //     && let Some(animation) = state.animations::<WorkspaceActive>()
        // {
        //     let workspaces = load_workspaces(state, layer);
        //     let row_box = row(workspaces.into_iter().map(|w| pill(w, animation).into())).spacing(2);

        //     return container(row_box)
        //         .id(self.id(id))
        //         .padding(Padding::from([4, 5]))
        //         .style(|_| pills_style(colors::COLOR3))
        //         .into();
        // };
        container("").into()
    }

    fn id(&self, id: WindowId) -> Id {
        Id::from(format!("{}-{}", self.namespace(), id))
    }

    fn display_id(&self) -> u32 {
        self.display_id
    }

    fn namespace(&self) -> String {
        "pill-workspace".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Top
    }
}

// fn pill<'a>(workspace: , animation: &'a WorkspaceActive) -> MouseArea<'a, AppEvent> {
//     let index = workspace.id;
//     let now = Instant::now();

//     let (width, color) = match animation.progress(&index, now) {
//         Some(progress) => (
//             PILL_HEIGHT + (PILL_WIDTH - PILL_HEIGHT) * progress,
//             mix_colors(colors::COLOR4, colors::COLOR3, progress),
//         ),
//         None => (PILL_HEIGHT, colors::COLOR4),
//     };

//     let click_event: HyprlandEvent = HyprlandSend::SwitchWorkspace(index).into();

//     mouse_area(
//         container("")
//             .height(PILL_HEIGHT)
//             .width(width)
//             .style(move |_: &Theme| container::Style {
//                 background: Some(color.into()),
//                 border: Border {
//                     radius: 1.0.into(),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             }),
//     )
//     .on_press(click_event.into())
// }

// fn load_workspaces(state: &AppState, layer: &LayerInfo) -> Vec<HyprWorkspace> {
//     if let Some(display) = state.displays.get(&layer.display_id())
//         && let Some(HyprWorkspaces(ws)) = state.data.get::<HyprWorkspaces>()
//     {
//         let d_name = display.name();
//         let mut workspaces = (ws.iter().filter(|(_, w)| w.monitor == d_name))
//             .map(|(_, w)| w.clone())
//             .collect::<Vec<_>>();
//         workspaces.sort_by_key(|w| w.id);
//         return workspaces;
//     }
//     vec![]
// }
