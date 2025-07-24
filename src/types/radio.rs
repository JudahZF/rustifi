use crate::{
    responses::stat::devices::RadioTable as RawRadio, utils::channel_conversion::freq_to_channel,
};

use std::fmt;

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

impl fmt::Display for RadioType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RadioType::N24 => write!(f, "N24"),
            RadioType::AC5 => write!(f, "AC5"),
            RadioType::AX6 => write!(f, "AX6"),
        }
    }
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
        if raw.is_11ac.is_some() {
            rf_type = RadioType::AC5;
        } else if raw.is_11ax.is_some() {
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
            channel_optimization_enabled: raw.channel_optimization_enabled.unwrap_or_default(),
            current_channel: match raw.channel.as_u64() {
                Some(c) => c.try_into().unwrap(),
                None => 0,
            },
            current_gain: raw.current_antenna_gain,
            has_dfs: raw.has_dfs.unwrap_or_default(),
            has_fccdfs: raw.has_fccdfs.unwrap_or_default(),
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
            min_rssi: raw.min_rssi.unwrap_or(-120),
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

impl From<String> for RadioType {
    fn from(raw: String) -> Self {
        match raw.as_str() {
            "N24" => RadioType::N24,
            "AC5" => RadioType::AC5,
            "AX6" => RadioType::AX6,
            _ => RadioType::N24,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::stat::devices::RadioTable;
    use serde_json::Value;

    #[test]
    fn test_radio_type_from_string() {
        assert_eq!(RadioType::from("N24".to_string()), RadioType::N24);
        assert_eq!(RadioType::from("AC5".to_string()), RadioType::AC5);
        assert_eq!(RadioType::from("AX6".to_string()), RadioType::AX6);
        assert_eq!(RadioType::from("invalid".to_string()), RadioType::N24);
    }

    #[test]
    fn test_radio_from_raw_basic() {
        let raw = RadioTable {
            antenna_gain: 5,
            builtin_antenna: true,
            builtin_ant_gain: 3,
            ht: Value::from(40),
            channel_optimization_enabled: Some(true),
            channel: Value::from(36),
            current_antenna_gain: 4,
            has_dfs: Some(true),
            has_fccdfs: Some(false),
            antenna_id: 1,
            is_11ac: Some(true),
            is_11ax: None,
            max_chan_cntr_frq: Some(5825),
            max_txpower: 23,
            min_chan_cntr_frq: Some(5180),
            min_txpower: 8,
            min_rssi: Some(-75),
            min_rssi_enabled: true,
            name: "radio0".to_string(),
            nss: 2,
            tx_power_mode: "high".to_string(),
            vwire_enabled: false,
            hard_noise_floor_enabled: Some(false),
            sens_level_enabled: Some(false),
            radio: "".to_string(),
            radio_caps: Some(0),
            radio_caps2: Some(0),
            has_ht160: Some(false),
            has_restricted_channels: Some(false),
            backup_channel: None,
            loadbalance_enabled: Some(false),
            sens_level: None,
        };

        let radio = Radio::from(raw);

        assert_eq!(radio.antenna_gain, 5);
        assert_eq!(radio.builtin_antenna, true);
        assert_eq!(radio.builtin_antenna_gain, 3);
        assert_eq!(radio.channel_width, 40);
        assert_eq!(radio.channel_optimization_enabled, true);
        assert_eq!(radio.current_channel, 36);
        assert_eq!(radio.current_gain, 4);
        assert_eq!(radio.has_dfs, true);
        assert_eq!(radio.has_fccdfs, false);
        assert_eq!(radio.antenna_id, 1);
        assert_eq!(radio.radio_type, RadioType::AC5);
        assert_eq!(radio.max_channel, 165); // 5825 MHz maps to channel 165
        assert_eq!(radio.max_power, 23);
        assert_eq!(radio.min_channel, 36); // 5170 MHz maps to channel 36
        assert_eq!(radio.min_power, 8);
        assert_eq!(radio.min_rssi, -75);
        assert_eq!(radio.min_rssi_enabled, true);
        assert_eq!(radio.name, "radio0");
        assert_eq!(radio.spacial_streams, 2);
        assert_eq!(radio.power_mode, RadioPowerMode::High);
    }

    #[test]
    fn test_radio_from_raw_defaults() {
        let raw = RadioTable {
            antenna_gain: 0,
            builtin_antenna: false,
            builtin_ant_gain: 0,
            ht: Value::Null,
            channel_optimization_enabled: None,
            channel: Value::Null,
            current_antenna_gain: 0,
            has_dfs: None,
            has_fccdfs: None,
            antenna_id: 0,
            is_11ac: None,
            is_11ax: None,
            max_chan_cntr_frq: None,
            max_txpower: 0,
            min_chan_cntr_frq: None,
            min_txpower: 0,
            min_rssi: None,
            min_rssi_enabled: false,
            name: "".to_string(),
            nss: 0,
            tx_power_mode: "invalid".to_string(),
            vwire_enabled: false,
            hard_noise_floor_enabled: None,
            sens_level_enabled: None,
            radio: "".to_string(),
            radio_caps: Some(0),
            radio_caps2: Some(0),
            has_ht160: None,
            has_restricted_channels: None,
            backup_channel: None,
            loadbalance_enabled: None,
            sens_level: None,
        };

        let radio = Radio::from(raw);

        assert_eq!(radio.channel_width, 0);
        assert_eq!(radio.channel_optimization_enabled, false);
        assert_eq!(radio.current_channel, 0);
        assert_eq!(radio.has_dfs, false);
        assert_eq!(radio.has_fccdfs, false);
        assert_eq!(radio.radio_type, RadioType::N24);
        assert_eq!(radio.max_channel, 0);
        assert_eq!(radio.min_channel, 0);
        assert_eq!(radio.min_rssi, -120);
        assert_eq!(radio.power_mode, RadioPowerMode::Unknown);
    }

    #[test]
    fn test_radio_type_ordering() {
        assert!(RadioType::N24 < RadioType::AC5);
        assert!(RadioType::AC5 < RadioType::AX6);
        assert!(RadioType::N24 < RadioType::AX6);
    }

    #[test]
    fn test_radio_power_mode_ordering() {
        assert!(RadioPowerMode::Low < RadioPowerMode::Medium);
        assert!(RadioPowerMode::Medium < RadioPowerMode::High);
        assert!(RadioPowerMode::Low < RadioPowerMode::High);
        assert!(RadioPowerMode::Low < RadioPowerMode::Unknown);
        assert!(RadioPowerMode::Medium < RadioPowerMode::Unknown);
        assert!(RadioPowerMode::High < RadioPowerMode::Unknown);
    }
}
