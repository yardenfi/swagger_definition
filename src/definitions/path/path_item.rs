use crate::OperationObject;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PathItem {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub get: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub put: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub options: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub head: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch: Option<OperationObject>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub trace: Option<OperationObject>
}

#[cfg(test)]
mod tests {
    use super::PathItem;
    use serde_json;
    use serde_json::Value;
    use serde_json::value::Value::Null;

    #[test]
    fn test_operations_arent_serialized_unless_specified_explicitly() {
        let path_item = PathItem {
            ..Default::default()
        };

        let serialized_str = serde_json::to_string(&path_item).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized_str).unwrap();
        for attribute in ["get", "post", "put", "delete", "options", "head", "trace"].iter() {
            assert_eq!(Null, deserialized[attribute]);
        }
    }
}

