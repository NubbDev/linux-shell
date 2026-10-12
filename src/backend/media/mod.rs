use tokio::time::{sleep, Duration};

use iced::futures::{
    SinkExt, channel::mpsc::Sender
};
use mpris::PlayerFinder;

use super::EventType;

mod spotify;

pub use spotify::{Spotify, SpotifyCoverArt, SpotifyStatus};

pub async fn run(mut sender: Sender<EventType>) {
    let finder = PlayerFinder::new().expect("mpris player finder failed");
    let mut spotify_id: Option<String> = None;
    let mut events: Vec<EventType> = Vec::new();
    println!("Starting media event listener...");
    loop {
        let Ok(players) = finder.find_all() else {
            sleep(Duration::from_secs(500)).await;
            continue;
        };
        for player in players.iter() {
            let identity = player.identity().to_lowercase();
            match identity.as_str() {
                "spotify" => {
                    if let Ok(e) = spotify::handle_event(&player, &mut spotify_id) {
                        events.extend(e);
                    }
                }
                "mozilla firefox" => {
                    // Firefox Handler
                    // println!("Firefox: {:?}", player);
                }
                _ => {}
            }
        }

        for event in events.drain(..) {
            if let Err(e) = sender.send(event).await {
                eprintln!("Failed to send event: {:?}", e);
            }
        }
        sleep(Duration::from_millis(250)).await;
    }
}
