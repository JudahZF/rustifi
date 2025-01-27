#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    bugfix: Option<u64>,
}

impl From<String> for Version {
    fn from(s: String) -> Self {
        let mut parts = s.split(".");
        let major = parts.next().unwrap().parse::<u64>().unwrap();
        let minor = parts.next().unwrap().parse::<u64>().unwrap();
        let patch = parts.next().unwrap().parse::<u64>().unwrap();
        if let Some(bugfix) = parts.next() {
            Version {
                major,
                minor,
                patch,
                bugfix: Some(bugfix.parse::<u64>().unwrap()),
            }
        } else {
            Version {
                major,
                minor,
                patch,
                bugfix: None,
            }
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        let mut parts = s.split(".");
        let major = parts.next().unwrap().parse::<u64>().unwrap();
        let minor = parts.next().unwrap().parse::<u64>().unwrap();
        let patch = parts.next().unwrap().parse::<u64>().unwrap();
        if let Some(bugfix) = parts.next() {
            Version {
                major,
                minor,
                patch,
                bugfix: Some(bugfix.parse::<u64>().unwrap()),
            }
        } else {
            Version {
                major,
                minor,
                patch,
                bugfix: None,
            }
        }
    }
}
