use crate::ParameterObject;
use crate::RequestBodyObject;
use crate::ValueOrRef;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OperationObject {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    // pub externalDocs:
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<ValueOrRef<Vec<ParameterObject>>>,
    pub request_body: Option<RequestBodyObject>,
//    pub responses:
//    pub callbacks:
    pub deprecated: Option<bool>
//    pub security:
}
