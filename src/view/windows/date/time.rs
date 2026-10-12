use iced::{
    Element, Font, Padding,
    widget::{Id, container, text},
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;

use crate::{DEFAULT_FONT, app::AppState, backend::TimeDate, colors, event::AppEvent};

use super::super::{WindowComponent, pills_style};

#[derive(Debug, Clone)]
pub struct TimePill {
    display_id: u32,
}

impl WindowComponent for TimePill {
    fn new(display_id: u32) -> Self {
        Self { display_id }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        let Some(time_date) = state.data.get::<TimeDate>() else {
            return container("").into();
        };
        let time = time_date.0.format("%I:%M %p").to_string();
        let text_item = text(time)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..DEFAULT_FONT
            })
            .size(16)
            .style(|_| text::Style {
                color: Some(colors::PRIMARY),
            });
        container(text_item)
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
