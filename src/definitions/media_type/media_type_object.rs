use serde_json::Value;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MediaTypeObject {
    pub schema: Option<Value>,
    pub example: Option<Value>
}

#[cfg(test)]
mod tests {
    use super::MediaTypeObject;
    use serde_json;

    #[test]
    pub fn creating_media_type_object() {
        MediaTypeObject {
            example: Some(json!({"example": "example value"})),
            schema: Some(json!({
                "type": "string"
            }))
        };
    }

    #[test]
    fn serializing_and_deserializing_media_type_object() {
        let media_type = MediaTypeObject {
            example: Some(json!({"example": "example value"})),
            schema: Some(json!({
                "type": "string"
            }))
        };

        let serialized = serde_json::to_string(&media_type).unwrap();
        let _: MediaTypeObject = serde_json::from_str(&serialized).unwrap();
    }
}
