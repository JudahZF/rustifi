use crate::{
    responses::stat::devices::RadioTable as RawRadio, utils::channel_conversion::freq_to_channel,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Radio {
    pub antenna_gain: u16,
    pub builtin_antenna: bool,
    pub builtin_antenna_gain: u16,
    pub channel_width: u16,
    pub channel_optimization_enabled: bool,
    pub current_channel: u16,
    pub current_gain: u16,
    pub has_dfs: bool,
    pub has_fccdfs: bool,
    pub antenna_id: i16,
    pub radio_type: RadioType,
    pub max_channel: u16,
    pub max_power: u16,
    pub min_channel: u16,
    pub min_power: u16,
    pub min_rssi: i16,
    pub min_rssi_enabled: bool,
    pub name: String,
    pub power_mode: RadioPowerMode,
    pub spacial_streams: u16,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum RadioType {
    #[default]
    N24,
    AC5,
    AX6,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum RadioPowerMode {
    Low,
    Medium,
    High,
    #[default]
    Unknown,
}

impl From<RawRadio> for Radio {
    fn from(raw: RawRadio) -> Self {
        let mut rf_type = RadioType::N24;
        if let Some(_) = raw.is_11ac {
            rf_type = RadioType::AC5;
        } else if let Some(_) = raw.is_11ax {
            rf_type = RadioType::AX6;
        }

        Radio {
            antenna_gain: raw.antenna_gain,
            builtin_antenna: raw.builtin_antenna,
            builtin_antenna_gain: raw.builtin_ant_gain,
            channel_width: match raw.ht.as_u64() {
                Some(c) => c.try_into().unwrap(),
                None => 0,
            },
            channel_optimization_enabled: match raw.channel_optimization_enabled {
                Some(enabled) => enabled,
                None => false,
            },
            current_channel: match raw.channel.as_u64() {
                Some(c) => c.try_into().unwrap(),
                None => 0,
            },
            current_gain: raw.current_antenna_gain,
            has_dfs: match raw.has_dfs {
                Some(enabled) => enabled,
                None => false,
            },
            has_fccdfs: match raw.has_fccdfs {
                Some(enabled) => enabled,
                None => false,
            },
            antenna_id: raw.antenna_id,
            radio_type: rf_type,
            max_channel: match raw.max_chan_cntr_frq {
                Some(c) => freq_to_channel(c),
                None => 0,
            },
            max_power: raw.max_txpower,
            min_channel: match raw.min_chan_cntr_frq {
                Some(c) => freq_to_channel(c),
                None => 0,
            },
            min_power: raw.min_txpower,
            min_rssi: match raw.min_rssi {
                Some(rssi) => rssi,
                None => -120,
            },
            min_rssi_enabled: raw.min_rssi_enabled,
            name: raw.name,
            spacial_streams: raw.nss,
            power_mode: match raw.tx_power_mode.as_str() {
                "low" => RadioPowerMode::Low,
                "medium" => RadioPowerMode::Medium,
                "high" => RadioPowerMode::High,
                _ => RadioPowerMode::Unknown,
            },
        }
    }
}
