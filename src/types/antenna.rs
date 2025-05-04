use crate::responses::stat::devices::AntennaTable as RawAntenna;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Antenna {
    default: bool,
    name: String,
    id: i64,
    wifi0_gain: Option<u16>,
    wifi1_gain: Option<u16>,
    wifi2_gain: Option<u16>,
    wifi3_gain: Option<u16>,
}

impl From<RawAntenna> for Antenna {
    fn from(raw: RawAntenna) -> Self {
        Antenna {
            default: raw.default.unwrap_or_default(),
            name: raw.name,
            id: raw.id,
            wifi0_gain: raw.wifi0_gain,
            wifi1_gain: raw.wifi1_gain,
            wifi2_gain: raw.wifi2_gain,
            wifi3_gain: raw.wifi3_gain,
        }
    }
}
