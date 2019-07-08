#[derive(Serialize, Deserialize, Debug)]
pub enum ParameterPlace {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "cookie")]
    Cookie,
    #[serde(rename = "body")]
    Body,
    #[serde(rename = "formData")]
    FormData
}

impl Default for ParameterPlace {
    fn default() -> Self {
        return ParameterPlace::Query;
    }
}


