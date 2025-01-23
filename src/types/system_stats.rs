use crate::responses::stat::devices::SystemStats as RawSystemStats;

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu: f32,
    pub memory: f32,
    pub uptime: u64,
}

impl SystemStats {
    pub fn from_raw(raw: Option<RawSystemStats>) -> SystemStats {
        match raw {
            Some(s) => SystemStats {
                cpu: s.cpu.parse::<f32>().unwrap(),
                memory: s.mem.parse::<f32>().unwrap(),
                uptime: s.uptime.parse::<u64>().unwrap(),
            },
            None => SystemStats {
                cpu: -1.0,
                memory: -1.0,
                uptime: 0,
            },
        }
    }
}
