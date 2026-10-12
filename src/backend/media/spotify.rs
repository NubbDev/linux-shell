use std::{
    error::Error,
    time::{Duration, Instant},
};

use iced::{Task, futures::{SinkExt, channel::mpsc::{Sender, UnboundedSender}}};
use mpris::{Metadata, Player};
use reqwest::Url;
use typemap_rev::{TypeMap, TypeMapKey};

use crate::{
    backend::{EventType, ImageSource},
    event::AppEvent,
};

#[derive(Debug, Clone)]
pub struct Spotify {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub album: Option<String>,
    pub cover_art: SpotifyCoverArt,
    pub duration: Duration,
}

impl TryFrom<&Metadata> for Spotify {
    type Error = &'static str;

    fn try_from(value: &Metadata) -> Result<Self, Self::Error> {
        let id = value
            .get("mpris:trackid")
            .and_then(|v| v.as_string())
            .and_then(|s| s.split("/").last());

        let title = value.get("xesam:title").and_then(|v| v.as_string());
        let artist = (value.get("xesam:artist").and_then(|v| v.as_array())).map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_string())
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        });
        let album = value.get("xesam:album").and_then(|v| v.as_string());
        let cover_art = value
            .get("mpris:artUrl")
            .and_then(|v| v.as_string().cloned())
            .map(SpotifyCoverArt::try_from)
            .transpose()?;

        let duration = value.get("mpris:length").and_then(|v| v.as_u64());

        Ok(Spotify {
            id: id.ok_or("Missing track ID")?.to_string(),
            title: title.ok_or("Missing title")?.to_string(),
            artists: artist.ok_or("Missing artists")?.to_vec(),
            album: album.map(|s| s.to_string()),
            cover_art: cover_art.ok_or("Missing cover art")?,
            duration: Duration::from_micros(duration.ok_or("Missing duration")?),
        })
    }
}

impl TypeMapKey for Spotify {
    type Value = Self;
}

impl super::super::Patch for Spotify {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(old_spotify) = data.get_mut::<Spotify>() {
            old_spotify.clone_from(self.as_ref());
        } else {
            data.insert::<Spotify>(*self);
        }

        Task::none()
    }
}

#[derive(Debug, Clone)]
pub struct SpotifyCoverArt(Url);

impl Into<String> for &SpotifyCoverArt {
    fn into(self) -> String {
        self.hash()
    }
}

impl TryFrom<String> for SpotifyCoverArt {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = Url::parse(&value).map_err(|_| "Invalid Spotify URL")?;
        Ok(SpotifyCoverArt(url))
    }
}

impl super::super::ImageSource for SpotifyCoverArt {
    fn url(&self) -> Url {
        self.0.clone()
    }
    fn hash(&self) -> String {
        "spotify_cover_art".to_string()
    }
}
#[derive(Debug, Clone)]
pub struct SpotifyStatus {
    pub identity: String,
    pub playing: bool,
    pub position: Duration,
    pub fetched_at: Instant,
}
impl SpotifyStatus {
    pub fn estimated_position(&self) -> Duration {
        self.position
    }
}

impl TypeMapKey for SpotifyStatus {
    type Value = Self;
}

impl super::super::Patch for SpotifyStatus {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(old_status) = data.get_mut::<SpotifyStatus>() {
            old_status.position = self.position;
            old_status.playing = self.playing;
        } else {
            data.insert::<SpotifyStatus>(*self);
        }
        Task::none()
    }
}

fn get_status(player: &Player) -> Result<SpotifyStatus, Box<dyn Error>> {
    let status = player.get_playback_status()?;
    let position = player.get_position_in_microseconds()?;
    Ok(SpotifyStatus {
        identity: player.identity().to_string(),
        playing: matches!(status, mpris::PlaybackStatus::Playing),
        position: Duration::from_micros(position),
        fetched_at: Instant::now(),
    })
}

fn get_metadata(player: &Player, old_id: &mut Option<String>) -> Result<Option<(Spotify, SpotifyCoverArt)>, Box<dyn Error>> {
    let metadata = player.get_metadata()?;
    let spotify = Spotify::try_from(&metadata)?;

    if old_id.as_ref() != Some(&spotify.id) {
        *old_id = Some(spotify.id.clone());
        let cover = spotify.cover_art.clone();
        return Ok(Some((spotify, cover)));
    }

    Ok(None)
}

pub fn handle_event(player: &Player, old_id: &mut Option<String>) -> Result<Vec<EventType>, Box<dyn Error>> {
    let mut events = Vec::new();
    let status = get_status(player)?;
    let is_playing = status.playing;
    events.push(EventType::data(status));

    if !is_playing {
        return Ok(events);
    }

    if let Some((spotify, cover)) = get_metadata(player, old_id)? {
        events.push(EventType::data(spotify));
        events.push(EventType::image(cover));
    }

    Ok(events)
}
