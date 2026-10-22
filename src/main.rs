use iced::Font;
use iced_layershell::Result;

mod animations;
mod app;
mod backend;
mod data;
mod event;
mod update;
mod view;

mod colors;

pub const DEFAULT_FONT: Font = Font::with_name("FiraMono Nerd Font Propo");

pub const SIDE_BAR_DISPLAYS: &[&str] = &["DP-1", "DP-2"];
pub const APP_DOCK_DISPLAYS: &[&str] = &["DP-1"];
pub const STATUS_BAR_DISPLAYS: &[&str] = &["DP-1", "DP-2"]; //
pub const AUDIO_SINKS: &[&str] = &["Games", "Chat", "Music", "Media"];

fn main() -> Result {
    env_logger::init();

    iced_layershell::daemon(app::run, name_space, update::run, view::run)
        .style(app::style)
        .settings(app::initial_settings())
        .subscription(backend::subscription)
        .run()
}

fn name_space() -> String {
    "shell".to_string()
}
