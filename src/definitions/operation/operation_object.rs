use crate::ParameterObject;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OperationObject {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    // pub externalDocs:
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<ParameterObject>>,
    //    pub requestBody:
//    pub responses:
//    pub callbacks:
    pub deprecated: Option<bool>
//    pub security:
}
