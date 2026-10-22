use iced::{
    Alignment, Color, Element, Font, Padding, Theme,
    alignment::Vertical,
    font::Weight,
    widget::{
        Id, Text, container, mouse_area,
        text::{self, Shaping, Wrapping},
    },
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;

use crate::{
    DEFAULT_FONT,
    animations::{AnimationableProgress, hover::HoverAnimations},
    app::AppState,
    colors,
    event::AppEvent,
    view::{
        components::set_hover,
        icons::{Icon, media::VolumeHighIcon},
    },
};

use super::super::{WindowComponent, pills_style};

#[derive(Debug, Clone)]
pub struct VolumePill {
    display: String,
}

impl WindowComponent for VolumePill {
    fn new(display: String) -> Self {
        Self { display }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        let hovers = state.animations::<HoverAnimations>();
        let now = std::time::Instant::now();
        let hover_id = format!("volume-button-{}", id).into();
        let progress = hovers
            .and_then(|h| h.progress(&hover_id, now))
            .unwrap_or(0.0);

        let color = colors::mix_colors(colors::COLOR5, colors::PRIMARY, progress);

        let content = container(create_icon(VolumeHighIcon, color))
            .align_x(Alignment::Center)
            .align_y(Vertical::Center)
            .height(20)
            .width(20)
            .style(move |_: &Theme| container::Style {
                background: None,
                ..Default::default()
            });

        let area = mouse_area(content)
            .on_enter(set_hover(&hover_id, true))
            .on_exit(set_hover(&hover_id, false));

        container(area)
            .id(self.id(id))
            .padding(Padding::from([4, 5]))
            .style(move |_| pills_style(color))
            .into()
    }

    fn id(&self, id: WindowId) -> Id {
        Id::from(format!("{}-{}", self.namespace(), id))
    }

    fn display(&self) -> &String {
        &self.display
    }

    fn namespace(&self) -> String {
        "pill-volume".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Top
    }
}

fn create_icon<'a, T: Icon>(icon: T, color: Color) -> Text<'a> {
    Text::new(icon.to_char())
        .font(Font {
            weight: Weight::Bold,
            ..DEFAULT_FONT
        })
        .align_x(Alignment::Center)
        .align_y(Vertical::Center)
        .wrapping(Wrapping::None)
        // .height(20)
        // .width(20)
        .line_height(1.0)
        .shaping(Shaping::Advanced)
        .size(16)
        .style(move |_: &Theme| text::Style { color: Some(color) })
}
