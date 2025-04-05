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
            default: match raw.default {
                Some(b) => b,
                None => false,
            },
            name: raw.name,
            id: raw.id,
            wifi0_gain: raw.wifi0_gain,
            wifi1_gain: raw.wifi1_gain,
            wifi2_gain: raw.wifi2_gain,
            wifi3_gain: raw.wifi3_gain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antenna_default() {
        let antenna = Antenna::default();
        assert!(!antenna.default);
        assert_eq!(antenna.name, "");
        assert_eq!(antenna.id, 0);
        assert_eq!(antenna.wifi0_gain, None);
        assert_eq!(antenna.wifi1_gain, None);
        assert_eq!(antenna.wifi2_gain, None);
        assert_eq!(antenna.wifi3_gain, None);
    }

    #[test]
    fn test_antenna_from_raw_complete() {
        let raw = RawAntenna {
            default: Some(true),
            name: "Standard (included)".to_string(),
            id: 4,
            wifi0_gain: Some(3),
            wifi1_gain: Some(4),
            wifi2_gain: Some(5),
            wifi3_gain: Some(6),
        };

        let antenna = Antenna::from(raw);

        assert!(antenna.default);
        assert_eq!(antenna.name, "Standard (included)");
        assert_eq!(antenna.id, 4);
        assert_eq!(antenna.wifi0_gain, Some(3));
        assert_eq!(antenna.wifi1_gain, Some(4));
        assert_eq!(antenna.wifi2_gain, Some(5));
        assert_eq!(antenna.wifi3_gain, Some(6));
    }

    #[test]
    fn test_antenna_from_raw_partial() {
        let raw = RawAntenna {
            default: None,
            name: "External Antenna".to_string(),
            id: 1,
            wifi0_gain: Some(2),
            wifi1_gain: Some(3),
            wifi2_gain: None,
            wifi3_gain: None,
        };

        let antenna = Antenna::from(raw);

        assert!(!antenna.default); // Should default to false when None
        assert_eq!(antenna.name, "External Antenna");
        assert_eq!(antenna.id, 1);
        assert_eq!(antenna.wifi0_gain, Some(2));
        assert_eq!(antenna.wifi1_gain, Some(3));
        assert_eq!(antenna.wifi2_gain, None);
        assert_eq!(antenna.wifi3_gain, None);
    }

    #[test]
    fn test_antenna_derived_traits() {
        let antenna1 = Antenna {
            default: true,
            name: "Test Antenna".to_string(),
            id: 1,
            wifi0_gain: Some(3),
            wifi1_gain: Some(4),
            wifi2_gain: None,
            wifi3_gain: None,
        };

        let antenna2 = antenna1.clone();

        // Test Clone and PartialEq
        assert_eq!(antenna1, antenna2);

        // Test Debug
        let debug_str = format!("{:?}", antenna1);
        assert!(debug_str.contains("Test Antenna"));
        assert!(debug_str.contains("wifi0_gain: Some(3)"));

        // Test Ord
        let antenna3 = Antenna {
            default: true,
            name: "Test Antenna".to_string(),
            id: 2, // Different ID
            wifi0_gain: Some(3),
            wifi1_gain: Some(4),
            wifi2_gain: None,
            wifi3_gain: None,
        };

        assert!(antenna1 < antenna3); // Should compare based on id
    }
}
