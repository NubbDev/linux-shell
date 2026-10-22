use std::time::Instant;

use iced::{
    Alignment, Color, Element, Font, Padding, Theme,
    alignment::Vertical,
    font::Weight,
    widget::{
        Column, Container, Id, MouseArea, Row, Slider, Text, container, mouse_area,
        text::{self, Shaping, Wrapping},
    },
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;
use pulsectl::controllers::types::DeviceInfo;

use crate::{
    DEFAULT_FONT,
    animations::{AnimationableProgress, hover::HoverAnimations},
    app::AppState,
    backend::AudioDevices,
    colors,
    event::AppEvent,
    view::{
        components::set_hover,
        icons::{Icon, media::VolumeHighIcon},
    },
};

use super::super::{WindowComponent, pills_style};

#[derive(Debug, Clone)]
pub struct VolumePanel {
    display: String,
}

impl WindowComponent for VolumePanel {
    fn new(display: String) -> Self {
        Self { display }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        let Some(audio_devices) = state.data.get::<AudioDevices>() else {
            println!("Spotify data not found in state");
            return Container::new("").into();
        };
        let hovers = state.animations::<HoverAnimations>();
        let now = std::time::Instant::now();

        let mut col = Column::new().spacing(6).align_x(Alignment::Center);

        for (idx, device) in audio_devices {
            col = col.push(volume_component(device, hovers, id));
        }

        container(col)
            .id(self.id(id))
            .padding(Padding::from([4, 5]))
            .style(move |_| pills_style(colors::COLOR5))
            .into()
    }

    fn id(&self, id: WindowId) -> Id {
        Id::from(format!("{}-{}", self.namespace(), id))
    }

    fn display(&self) -> &String {
        &self.display
    }

    fn namespace(&self) -> String {
        "window-volume".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Top
    }
}

fn volume_component<'a>(
    device: &'a DeviceInfo,
    hovers: Option<&'a HoverAnimations>,
    id: WindowId,
) -> MouseArea<'a, AppEvent> {
    let hover_id = format!("volume-{}-{}", id, device.index).into();
    let now = Instant::now();
    let progress = hovers
        .and_then(|h| h.progress(&hover_id, now))
        .unwrap_or(0.0);
    let icon = create_icon(VolumeHighIcon, colors::COLOR5);

    let name = device.name.as_ref().map(|n| create_name(n, colors::COLOR5));

    let row = Row::new()
        .spacing(6)
        .align_y(Vertical::Center)
        .push(icon)
        .push(name);

    let slider = create_bar();

    let col = Column::new()
        .spacing(4)
        .align_x(Alignment::Center)
        .push(row)
        .push(slider);

    MouseArea::new(container(col).padding(Padding::from([4, 5])))
        .on_enter(set_hover(&hover_id, true))
        .on_exit(set_hover(&hover_id, false))
}

fn create_name<'a>(name: &'a str, color: Color) -> Text<'a> {
    Text::new(name)
        .font(Font {
            weight: Weight::Bold,
            ..DEFAULT_FONT
        })
        .align_x(Alignment::Center)
        .align_y(Vertical::Center)
        .wrapping(Wrapping::None)
        .line_height(1.0)
        .size(16)
        .style(move |_: &Theme| text::Style {
            color: Some(colors::COLOR5),
        })
}

fn create_bar<'a>() -> Slider<'a, f32, AppEvent> {
    Slider::new(0.0..=100.0, 50.0, handle_slider_change)
        .step(1f32)
        .width(200)
    // .style(move |_: &Theme| slider_style(colors::COLOR5))
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

fn handle_slider_change(value: f32) -> AppEvent {
    // Here you would implement the logic to change the system volume
    // For now, we just return an empty event
    AppEvent::Empty
}
