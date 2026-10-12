use iced::{
    futures::{
        SinkExt, channel::mpsc::Sender
    },
};
use tokio::time::{sleep, Duration};
mod cpu;
mod disk;
mod gpu;
mod memory;
mod network;

// pub use disk::Disk;
pub use cpu::Cpu;
pub use gpu::Gpu;
pub use memory::Memory;

use crate::{backend::EventType};

pub async fn run(mut sender: Sender<EventType>) {
    let mut cpu = Cpu::default();
    let mut memory = Memory::default();
    let mut gpu = Gpu::default();

    println!("Starting system event listener...");
    loop {
        cpu.poll().await;
        memory.poll().await;
        gpu.poll().await;

        if sender.send(EventType::data(cpu.clone())).await.is_err() {
            break;
        }
        if sender.send(EventType::data(memory.clone())).await.is_err() {
            break;
        }
        if sender.send(EventType::data(gpu.clone())).await.is_err() {
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }
}

pub(self) fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let bytes_f64 = bytes as f64;

    if bytes_f64 >= TB {
        format!("{:.2} TB", bytes_f64 / TB)
    } else if bytes_f64 >= GB {
        format!("{:.2} GB", bytes_f64 / GB)
    } else if bytes_f64 >= MB {
        format!("{:.2} MB", bytes_f64 / MB)
    } else if bytes_f64 >= KB {
        format!("{:.2} KB", bytes_f64 / KB)
    } else {
        format!("{} B", bytes)
    }
}
