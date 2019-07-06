use super::contact_info::SwaggerContactInfo;
use super::licence_info::SwaggerLicenceInfo;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InfoObject {
    pub version: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub contact: Option<SwaggerContactInfo>,
    pub license: Option<SwaggerLicenceInfo>
}