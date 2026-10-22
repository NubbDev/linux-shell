use iced::{
    Color, Element, Font, Padding, Shadow, Theme,
    alignment::Vertical,
    font::Weight,
    widget::{
        Container, Id, Image, MouseArea, Row, Text, container,
        text::{self, Alignment, Shaping, Wrapping},
    },
    window::Id as WindowId,
};
use iced_layershell::reexport::Anchor;

use crate::{
    DEFAULT_FONT,
    animations::{AnimationableProgress, hover::HoverAnimations, marquee::MarqueeKey},
    app::AppState,
    backend::{Spotify, SpotifyStatus},
    colors::{self, mix_colors},
    event::{AppEvent, MediaEvent},
    view::{
        Identifiers,
        components::{marquee, set_hover, window},
        icons::{
            Icon,
            media::{BackwardIcon, ForwardIcon, PauseIcon, PlayIcon},
        },
    },
};

use super::super::{WindowComponent, pills_style};

const TITLE_MARQUEE: &str = "song_text";
const ICON_SIZE: u32 = 20;

#[derive(Debug, Clone)]
pub struct SpotifyPill {
    display: String,
}

impl WindowComponent for SpotifyPill {
    fn new(display: String) -> Self {
        Self { display }
    }

    fn view<'a>(&self, id: WindowId, state: &'a AppState) -> Element<'a, AppEvent> {
        let (Some(song), Some(status)) = (
            state.data.get::<Spotify>(),
            state.data.get::<SpotifyStatus>(),
        ) else {
            return window("").into();
        };

        let hovers = state.animations::<HoverAnimations>();

        let mut content = Row::new().spacing(4).align_y(Vertical::Center);
        if let Some(cover) = state.images.get(&song.cover_art) {
            let cover_image = Image::new(cover.clone()).width(ICON_SIZE).height(ICON_SIZE);
            content = content.push(cover_image);
        }

        content = content
            .push(spotify_text(id, song))
            .push(media_control(status, id, hovers));

        window(
            Container::new(content)
                .padding(Padding::from([4, 5]))
                .style(|_| pills_style(colors::SUCCESS)),
        )
        .id(self.id(id))
        .into()
    }

    fn display(&self) -> &String {
        &self.display
    }

    fn namespace(&self) -> String {
        "pill-spotify".to_string()
    }

    fn anchor(&self) -> Anchor {
        Anchor::Top
    }

    fn identifiers(&self) -> Vec<Identifiers> {
        vec![Identifiers::marquee(TITLE_MARQUEE)]
    }
}

fn spotify_text(id: WindowId, song: &Spotify) -> Container<'_, AppEvent> {
    let key = MarqueeKey::new(id, TITLE_MARQUEE);
    let mut content = Row::new().spacing(4).align_y(Vertical::Center);
    let title = widget_text(&song.title, Weight::Bold);
    content = content.push(title);

    let seperator = widget_text("|", Weight::Bold);

    if let Some(artist) = song.artists.first() {
        let artists = widget_text(artist, Weight::Normal);
        content = content.push(seperator);
        content = content.push(artists);
    }

    marquee(&key, content).max_width(150).into()
}

fn widget_text(input: &str, weight: Weight) -> Element<'_, AppEvent> {
    let font = Font {
        weight,
        ..DEFAULT_FONT
    };
    let style = |_: &Theme| text::Style {
        color: Some(colors::SUCCESS),
    };
    Text::new(input)
        .font(font)
        .size(14)
        .wrapping(Wrapping::None)
        .style(style)
        .into()
}

fn media_control<'a>(
    status: &'a SpotifyStatus,
    id: WindowId,
    hovers: Option<&HoverAnimations>,
) -> Row<'a, AppEvent> {
    let spotify_iden = status.identity.clone();

    let rev_id = format!("spotify-rev-{}", id).into();
    let play_id = format!("spotify-play-{}", id).into();
    let fwd_id = format!("spotify-fwd-{}", id).into();

    let rev_button = create_button(rev_id, BackwardIcon, hovers).on_press(rev_event(&spotify_iden));

    let play_button = if status.playing {
        create_button(play_id, PauseIcon, hovers)
    } else {
        create_button(play_id, PlayIcon, hovers)
    }
    .on_press(play_event(&spotify_iden));

    let fwd_button = create_button(fwd_id, ForwardIcon, hovers).on_press(fwd_event(&spotify_iden));

    //   
    //   
    (Row::new().spacing(10).align_y(Vertical::Center))
        .push(rev_button)
        .push(play_button)
        .push(fwd_button)
}

fn create_button<'a, I: Icon>(
    id: Id,
    icon: I,
    hovers: Option<&HoverAnimations>,
) -> MouseArea<'a, AppEvent> {
    let now = std::time::Instant::now();
    let (dx, dy) = icon.optical_offset();
    let progress = hovers.and_then(|h| h.progress(&id, now)).unwrap_or(0.0);
    let background = mix_colors(Color::TRANSPARENT, colors::COLOR3, progress);

    let icon_text = create_icon(icon);

    let shadow = Shadow {
        color: Color::BLACK,
        offset: iced::Vector::new(0.0, progress * 1.0),
        blur_radius: 1.0,
    };

    let content = container(icon_text)
        .align_x(Alignment::Center)
        .align_y(Vertical::Center)
        .style(move |_: &Theme| container::Style {
            background: None,
            shadow,
            ..Default::default()
        })
        .padding(Padding {
            top: (dy).max(0.0),
            bottom: (-dy).max(0.0),
            left: (dx).max(0.0),
            right: (-dx).max(0.0),
        });

    MouseArea::new(content)
        .on_enter(set_hover(&id, true))
        .on_exit(set_hover(&id, false))
}

fn create_icon<'a, T: Icon>(icon: T) -> Text<'a> {
    let color = Some(colors::SUCCESS);
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
        .style(move |_: &Theme| text::Style { color })
}

fn rev_event(iden: &str) -> AppEvent {
    AppEvent::Media(MediaEvent::Previous(iden.to_string()))
}

fn play_event(iden: &str) -> AppEvent {
    AppEvent::Media(MediaEvent::PlayPause(iden.to_string()))
}

fn fwd_event(iden: &str) -> AppEvent {
    AppEvent::Media(MediaEvent::Next(iden.to_string()))
}
