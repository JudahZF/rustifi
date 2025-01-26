use crate::responses::stat::devices::SystemStats as RawSystemStats;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct SystemStats {
    pub cpu: f32,
    pub memory: f32,
    pub uptime: u64,
}

impl SystemStats {
    pub fn new(cpu: f32, memory: f32, uptime: u64) -> Self {
        SystemStats {
            cpu,
            memory,
            uptime,
        }
    }
}

impl From<RawSystemStats> for SystemStats {
    fn from(raw: RawSystemStats) -> Self {
        SystemStats {
            cpu: raw.cpu.parse::<f32>().unwrap(),
            memory: raw.mem.parse::<f32>().unwrap(),
            uptime: raw.uptime.parse::<u64>().unwrap(),
        }
    }
}

impl Display for SystemStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU: {}\nMemory: {}\nUptime: {}",
            self.cpu, self.memory, self.uptime
        )
    }
}
