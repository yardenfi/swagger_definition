use crate::InfoObject;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SwaggerDefinition {
    pub swagger: String,
    pub info: InfoObject,
    pub paths: crate::Paths,
    pub host: Option<String>,
    pub schemas: Option<Vec<String>>,
}
