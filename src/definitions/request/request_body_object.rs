#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequestBodyObject {
    pub description: Option<String>,
    pub required: Option<bool>
}
