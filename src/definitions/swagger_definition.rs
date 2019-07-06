use crate::InfoObject;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SwaggerDefinition {
    pub swagger: String,
    pub info: InfoObject,
    pub paths: crate::Paths,
    pub host: Option<String>,
    pub schemas: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use crate::SwaggerDefinition;
    use crate::InfoObject;
    use std::collections::HashMap;

    #[test]
    fn create_swagger_definition() {
        SwaggerDefinition {
            swagger: "2.0".to_owned(),
            info: InfoObject {
                version: "0.0.0".to_string(),
                title: "Title".to_string(),
                ..Default::default()
            },
            paths: HashMap::new(),
            ..Default::default()
        };
    }
}
