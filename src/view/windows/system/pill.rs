use iced::{
    Color, Element, Font, Padding, Theme,
    alignment::Vertical,
    font::Weight,
    widget::{
        Row, Text, container,
        text::{self, Wrapping},
    },
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;

use crate::{
    DEFAULT_FONT,
    app::AppState,
    backend::{Cpu, Gpu, Memory},
    colors,
    event::AppEvent,
    view::icons::{
        Icon,
        system::{CpuIcon, GpuIcon, MemoryIcon},
    },
};

const ICON_SIZE: u32 = 20;

use super::super::{WindowComponent, pills_style};

#[derive(Debug, Clone)]
pub struct SystemPill {
    display_id: u32,
}

impl WindowComponent for SystemPill {
    fn new(display_id: u32) -> Self {
        Self { display_id }
    }
    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        let (Some(cpu), Some(memory), Some(gpu)) = (
            state.data.get::<Cpu>(),
            state.data.get::<Memory>(),
            state.data.get::<Gpu>(),
        ) else {
            return container("").into();
        };


        let mut system_stats = Row::new().spacing(6).align_y(Vertical::Center);
        system_stats = system_stats.push(create_stat(CpuIcon, cpu.usage() * 100.0));
        system_stats = system_stats.push(create_stat(MemoryIcon, memory.usage() * 100.0));
        system_stats = system_stats.push(create_stat(GpuIcon, gpu.usage()));

        container(system_stats)
            .id(self.id(id))
            .padding(Padding::from([4, 5]))
            .style(|_| pills_style(colors::COLOR5))
            .into()
    }

    fn display_id(&self) -> u32 {
        self.display_id
    }

    fn namespace(&self) -> String {
        "pill-system".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Top
    }
}

fn create_stat<'a, T: Icon>(icon: T, value: f32) -> Text<'a> {
    let color = Some(color_for_value(value));

    Text::new(format!("{} {:.0}%", icon, value))
        .font(Font {
            weight: Weight::Bold,
            ..DEFAULT_FONT
        })
        .wrapping(Wrapping::None)
        .size(16)
        .style(move |_: &Theme| text::Style { color })
}

fn color_for_value(value: f32) -> Color {
    if value >= 90.0 {
        colors::COLOR1
    } else if value >= 80.0 {
        colors::DANGER
    } else if value >= 70.0 {
        colors::COLOR3
    } else if value >= 50.0 {
        colors::WARNING
    } else {
        colors::COLOR5
    }
}
