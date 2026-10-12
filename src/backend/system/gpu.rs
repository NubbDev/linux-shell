use regex::Regex;
use std::process;
use tokio::fs;

use crate::{backend::Patch, event::AppEvent};
use iced::Task;
use typemap_rev::{TypeMap, TypeMapKey};

const GPU_VRAM_USED_PATH: &str = "/sys/class/drm/card1/device/mem_info_vram_used";
const GPU_VRAM_TOTAL_PATH: &str = "/sys/class/drm/card1/device/mem_info_vram_total";
const GPU_USAGE_PATH: &str = "/sys/class/drm/card1/device/gpu_busy_percent";

#[derive(Debug, Clone)]
pub struct Gpu {
    name: String,
    vram_total: u64,
    vram_used: u64,
    usage: u32,
}

impl std::fmt::Display for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for Gpu {
    fn default() -> Self {
        let reg = Regex::new(r"\[([^\[\]]+)\]\s*$").unwrap();
        let mut name_proc = process::Command::new("sh");
        name_proc.arg("-c").arg(
            "lspci -vmm | grep -A3 'VGA compatible controller' | grep '^Device:' | cut -d: -f2",
        );

        let name = match name_proc.output() {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                (reg.captures(&output_str).and_then(|s| s.get(1)))
                    .map_or("Unknown GPU", |m| m.as_str())
                    .into()
            }
            Err(_) => "Unknown GPU".to_string(),
        };

        Self {
            name,
            vram_total: 0,
            vram_used: 0,
            usage: 0,
        }
    }
}

impl Gpu {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn vram_total(&self) -> u64 {
        self.vram_total
    }

    pub fn vram_used(&self) -> u64 {
        self.vram_used
    }

    pub fn usage(&self) -> f32 {
        self.usage as f32
    }

    pub fn vram_total_formatted(&self) -> String {
        super::format_bytes(self.vram_total)
    }

    pub fn vram_used_formatted(&self) -> String {
        super::format_bytes(self.vram_used)
    }

    pub async fn poll(&mut self) {
        self.poll_vram_used().await;
        self.poll_vram_total().await;
        self.poll_usage().await;
    }

    async fn poll_vram_used(&mut self) {
        if let Ok(vram_used_str) = fs::read_to_string(GPU_VRAM_USED_PATH).await
            && let Ok(vram_used) = vram_used_str.trim().parse::<u64>()
        {
            self.vram_used = vram_used;
        }
    }

    async fn poll_vram_total(&mut self) {
        if let Ok(vram_total_str) = fs::read_to_string(GPU_VRAM_TOTAL_PATH).await
            && let Ok(vram_total) = vram_total_str.trim().parse::<u64>()
        {
            self.vram_total = vram_total;
        }
    }

    async fn poll_usage(&mut self) {
        if let Ok(usage_str) = fs::read_to_string(GPU_USAGE_PATH).await
            && let Ok(usage) = usage_str.trim().parse::<u32>()
        {
            self.usage = usage;
        }
    }
}

impl TypeMapKey for Gpu {
    type Value = Gpu;
}

impl Patch for Gpu {
    fn apply(self: Box<Self>, data: &mut TypeMap) -> Task<AppEvent> {
        if let Some(gpu) = data.get_mut::<Gpu>() {
            *gpu = *self;
        } else {
            data.insert::<Gpu>(*self);
        }
        Task::none()
    }
}
