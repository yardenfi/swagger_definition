use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ExampleObject {
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "exampleValue", skip_serializing_if="Option::is_none")]
    pub external_value: Option<String>
}

mod tests {
    use crate::ExampleObject;

    #[test]
    pub fn creating_example_object() {
        ExampleObject {
            summary: Some("This is an example".to_owned()),
            description: Some("Example description".to_owned()),
            value: Some(json!({"i": "am alive"})),
            external_value: None
        };
    }

    #[test]
    pub fn serialize_deserialize_example_object() {
        let example_object = ExampleObject {
            summary: Some("This is an example".to_owned()),
            description: Some("Example description".to_owned()),
            value: Some(json!({"i": "am alive"})),
            external_value: None
        };

        let serialized_str = serde_json::to_string(&example_object).unwrap();
        let _: ExampleObject = serde_json::from_str(&serialized_str).unwrap();
    }

    #[test]
    pub fn deserialize_yaml() {
        let serialized = r#"
summary: This is a summary
description: This is the description
value:
    i: am alive
        "#.to_owned();

        let deserialized: ExampleObject = serde_yaml::from_str(&serialized).unwrap();
        let expected = ExampleObject {
            summary: Some("This is a summary".to_owned()),
            description: Some("This is the description".to_owned()),
            value: Some(json!({"i": "am alive"})),
            external_value: None
        };

        assert_eq!(deserialized, expected);
    }
}
