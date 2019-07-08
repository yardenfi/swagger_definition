#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ref {
    #[serde(rename = "query")]
    pub ref_: String
}
