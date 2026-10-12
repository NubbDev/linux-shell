use iced::{
    Border, Color, Element, Font, Length, Padding, Theme,
    alignment::Vertical,
    font::Weight,
    widget::{Id, ProgressBar, container, progress_bar, stack, text::Alignment},
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;

use crate::{
    DEFAULT_FONT,
    app::AppState,
    backend::{Spotify, SpotifyStatus},
    colors,
    event::AppEvent,
    view::components::SplitText,
};

use super::super::{WindowComponent, pills_style};

const BAR_WIDTH: f32 = 120.0;
const BAR_HEIGHT: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct SpotifyProgress {
    display_id: u32,
}

impl WindowComponent for SpotifyProgress {
    fn new(display_id: u32) -> Self {
        Self { display_id }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        let Some(status) = state.data.get::<SpotifyStatus>() else {
            return container("").into();
        };
        let Some(data) = state.data.get::<Spotify>() else {
            return container("").into();
        };
        let duration = data.duration.as_millis() as u64;
        let current_pos = status.estimated_position().as_millis() as u64;

        let progress = if duration > 0 {
            ((current_pos as f32) / (duration as f32)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let label = format!(
            "{} / {}",
            format_duration(current_pos),
            format_duration(duration)
        );
        let label_text = SplitText::new(label, progress * BAR_WIDTH)
            .width(Length::Fixed(BAR_WIDTH))
            .height(Length::Fixed(BAR_HEIGHT))
            .size(16)
            .horizontal_alignment(Alignment::Center)
            .vertical_alignment(Vertical::Center)
            .color_before(colors::BACKGROUND.scale_alpha(0.9))
            .color_after(colors::PRIMARY)
            .font(Font {
                weight: Weight::Bold,
                ..DEFAULT_FONT
            });

        let bar = ProgressBar::new(0.0..=duration as f32, current_pos.min(duration) as f32)
            .length(BAR_WIDTH)
            .girth(BAR_HEIGHT)
            .style(|_: &Theme| progress_bar::Style {
                background: Color::TRANSPARENT.into(),
                bar: colors::PRIMARY.into(),
                border: Border {
                    color: colors::PRIMARY,
                    width: 1.0,
                    radius: 2.0.into(),
                },
            });

        container(stack![bar, label_text])
            .id(self.id(id))
            .padding(Padding::from([4, 5]))
            .style(|_| pills_style(colors::PRIMARY))
            .into()
    }

    fn id(&self, id: WindowId) -> Id {
        Id::from(format!("{}-{}", self.namespace(), id))
    }

    fn display_id(&self) -> u32 {
        self.display_id
    }

    fn namespace(&self) -> String {
        "pill-time".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Top
    }
}

fn format_duration(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
