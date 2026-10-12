use std::thread;

use iced::{
    Subscription, Task,
    futures::{
        StreamExt,
        channel::mpsc::{Sender, unbounded},
        stream::BoxStream,
    },
};
use indexmap::IndexMap;
use pulsectl::controllers::{DeviceControl, SourceController, types::DeviceInfo};
use typemap_rev::{TypeMap, TypeMapKey};

use crate::{AUDIO_SINKS, backend::EventType, event::AppEvent};

fn volume() {
    let (tx, mut rx) = unbounded::<EventType>();

    println!("Managing volume");
    thread::spawn(move || {
        let mut source_handler = SourceController::create().unwrap();

        let device_outputs = source_handler.list_devices().unwrap();

        loop {
            let devices = (device_outputs.iter())
                .filter(|d| {
                    for sink in AUDIO_SINKS {
                        if let Some(name) = d.name.as_ref() {
                            if name.contains(&format!("{}-Out", sink)) {
                                return true;
                            }
                        }
                    }
                    false
                })
                .map(Clone::clone)
                .collect::<Vec<_>>();
            let _ = tx.unbounded_send(EventType::data(AudioDevices(devices)));
        }
    });
}

#[derive(Debug, Clone)]
pub struct AudioDevices(Vec<DeviceInfo>);

impl TypeMapKey for AudioDevices {
    type Value = IndexMap<u32, DeviceInfo>;
}

impl super::Patch for AudioDevices {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(devices) = data.get_mut::<Self>() {
            for device in self.0 {
                devices.insert(device.index, device);
            }
            for d in devices.iter() {
                println!("Device {}: {:?}", d.0, d.1.name);
            }
            println!("\n")
        } else {
            let mut map = IndexMap::new();
            for device in self.0 {
                map.insert(device.index, device);
            }
            data.insert::<AudioDevices>(map);
        }

        Task::none()
    }
}
