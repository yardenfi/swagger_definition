use crate::definitions::parameter::parameter_place::ParameterPlace;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParameterObject {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: ParameterPlace,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>
}