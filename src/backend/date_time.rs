use tokio::time::{Duration, sleep};

use chrono::{DateTime, Local, Timelike};
use iced::{
    Task,
    futures::{SinkExt, channel::mpsc::Sender},
};
use typemap_rev::{TypeMap, TypeMapKey};

use crate::event::AppEvent;

pub async fn run(mut sender: Sender<super::EventType>) {
    let mut old_min = 100;
    loop {
        let current_date = Local::now();
        if current_date.minute() != old_min {
            old_min = current_date.minute();
            if sender
                .send(super::EventType::data(TimeDate(current_date)))
                .await
                .is_err()
            {
                break;
            }
        }
        sleep(delay_until_next_minute(current_date)).await;
    }
}

fn delay_until_next_minute(now: DateTime<Local>) -> Duration {
    let seconds_into_min = now.second();
    let millis_into_second = now.timestamp_subsec_millis();

    let millis_until_next_min = (60 - seconds_into_min) * 1000 - millis_into_second;
    Duration::from_millis(millis_until_next_min as u64)
}

#[derive(Debug, Clone)]
pub struct TimeDate(pub DateTime<Local>);

impl TypeMapKey for TimeDate {
    type Value = Self;
}

impl super::Patch for TimeDate {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(time_date) = data.get_mut::<TimeDate>() {
            time_date.0 = *&self.0;
        } else {
            data.insert::<TimeDate>(*self);
        }
        Task::none()
    }
}
