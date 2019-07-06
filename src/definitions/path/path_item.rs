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