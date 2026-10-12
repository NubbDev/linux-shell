use tokio::fs;

use regex::Regex;

use crate::{backend::Patch, event::AppEvent};
use iced::Task;
use typemap_rev::{TypeMap, TypeMapKey};

const MEMORY_PATH: &str = "/proc/meminfo";

#[derive(Debug, Clone)]
pub struct Memory {
    total: u64,
    used: u64,
    usage: f32,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            total: 0,
            used: 0,
            usage: 0.0,
        }
    }
}

impl Memory {
    pub fn total(&self) -> u64 {
        self.total
    }
    pub fn used(&self) -> u64 {
        self.used
    }
    pub fn total_formatted(&self) -> String {
        super::format_bytes(self.total)
    }
    pub fn used_formatted(&self) -> String {
        super::format_bytes(self.used)
    }

    pub fn usage(&self) -> f32 {
        self.usage
    }

    pub async fn poll(&mut self) {
        let Ok(meminfo_text) = fs::read_to_string(MEMORY_PATH).await else {
            eprintln!("Failed to read memory info from {}", MEMORY_PATH);
            return;
        };

        let mut total_kb = 0;
        let mut avail_kb = 0;
        let reg = Regex::new(r"(\d+)").unwrap();
        for line in meminfo_text.trim().lines() {
            if line.starts_with("MemTotal:") {
                if let Some(caps) = reg.captures(line)
                    && let Some(m) = caps.get(1)
                {
                    total_kb = m.as_str().parse::<u64>().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(caps) = reg.captures(line)
                    && let Some(m) = caps.get(1)
                {
                    avail_kb = m.as_str().parse::<u64>().unwrap_or(0);
                }
            }
        }

        if total_kb > 0 {
            self.total = total_kb * 1024; // Convert to bytes
            self.used = (total_kb - avail_kb) * 1024; // Convert to bytes
            self.usage = self.used as f32 / self.total as f32;
        } else {
            eprintln!("Failed to parse memory info from {}", MEMORY_PATH);
        }
    }
}

impl TypeMapKey for Memory {
    type Value = Self;
}

impl Patch for Memory {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(memory) = data.get_mut::<Memory>() {
            *memory = *self;
        } else {
            data.insert::<Memory>(*self);
        }
        Task::none()
    }
}
