use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Temperature {
    pub name: String,
    pub type_str: String,
    pub value: f32,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nType: {}\nValue: {}",
            self.name, self.type_str, self.value
        )
    }
}
