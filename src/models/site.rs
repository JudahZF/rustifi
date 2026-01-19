use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Site {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub attr_no_delete: Option<bool>,
    #[serde(default)]
    pub attr_hidden_id: Option<String>,
}

impl Site {
    pub fn display_name(&self) -> &str {
        self.desc
            .as_ref()
            .filter(|d| !d.is_empty())
            .map_or(&self.name, |d| d.as_str())
    }
}
