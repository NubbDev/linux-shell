#[derive(Debug, Clone)]
pub enum MediaEvent {
    PlayPause(String),
    Next(String),
    Previous(String),
    Stop(String),
    None,
}

impl MediaEvent {
    pub fn get_name(&self) -> Option<&String> {
        match self {
            MediaEvent::PlayPause(name) => Some(name),
            MediaEvent::Next(name) => Some(name),
            MediaEvent::Previous(name) => Some(name),
            MediaEvent::Stop(name) => Some(name),
            MediaEvent::None => None,
        }
    }
}

impl From<MediaEvent> for super::AppEvent {
    fn from(media: MediaEvent) -> Self {
        super::AppEvent::Media(media)
    }
}
