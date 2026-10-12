use tokio::fs;

use iced::Task;
use regex::Regex;
use typemap_rev::{TypeMap, TypeMapKey};

use crate::{backend::Patch, event::AppEvent};

const STAT_PATH: &str = "/proc/stat";
const FREQ_PATH: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq";
const CPU_INFO_PATH: &str = "/proc/cpuinfo";

#[derive(Debug, Clone)]
pub struct Cpu {
    name: String,
    usage: f32,
    speed: f32,
    prev_usage: Option<(u64, u64)>, // (prev_idle, prev_total)
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name,)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        // /model name\s*:\s*(.+)/
        let reg = Regex::new(r"model name\s*:\s*(.+)").unwrap();
        let name = std::fs::read_to_string(CPU_INFO_PATH)
            .ok()
            .and_then(|info| {
                reg.captures(&info)
                    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            })
            .unwrap_or("Unknown CPU".to_string());

        Cpu {
            name,
            usage: 0.0,
            speed: 0.0,
            prev_usage: None,
        }
    }
}

impl Cpu {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn usage(&self) -> f32 {
        self.usage
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn speed_formatted(&self) -> String {
        if self.speed >= 1000.0 {
            return format!("{:.2} GHz", self.speed / 1000.0);
        }
        format!("{:.2} MHz", self.speed)
    }

    pub async fn poll(&mut self) {
        self.poll_usage().await;
        self.poll_speed().await;
    }
    async fn poll_usage(&mut self) {
        let Ok(stat_text) = fs::read_to_string(STAT_PATH).await else {
            return;
        };
        let Some(cpu_line) = stat_text.trim().lines().find(|l| l.starts_with("cpu ")) else {
            return;
        };
        let cpu_numbers = (cpu_line.trim().split_whitespace().skip(1))
            .map(&str::parse::<u64>)
            .flatten()
            .collect::<Vec<u64>>();

        let idle = cpu_numbers[3] + cpu_numbers[4]; // idle + iowait
        let total = cpu_numbers.iter().sum::<u64>();

        self.usage = if let Some((prev_idle, prev_total)) = self.prev_usage {
            let idle_diff = idle - prev_idle;
            let total_diff = total - prev_total;
            if total_diff != 0 {
                1.0 - (idle_diff as f32 / total_diff as f32)
            } else {
                0.0
            }
        } else {
            0.0
        };

        self.prev_usage = Some((idle, total));
    }

    async fn poll_speed(&mut self) {
        let Ok(freq_text) = fs::read_to_string(FREQ_PATH).await else {
            return;
        };
        let freq_khz = freq_text.trim().parse::<f32>().unwrap_or(0.0);
        self.speed = freq_khz / 1000.0; // Convert kHz to MHz
    }
}

impl TypeMapKey for Cpu {
    type Value = Cpu;
}

impl Patch for Cpu {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(cpu) = data.get_mut::<Cpu>() {
            *cpu = *self;
        } else {
            data.insert::<Cpu>(*self);
        }
        Task::none()
    }
}
