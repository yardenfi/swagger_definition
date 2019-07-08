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
    pub parameters: Option<Vec<ValueOrRef<ParameterObject>>>,
    pub request_body: Option<ValueOrRef<RequestBodyObject>>,
    //    pub responses:
//    pub callbacks:
    pub deprecated: Option<bool>,
//    pub security:
}

#[cfg(test)]
mod tests {
    use crate::OperationObject;

    #[test]
    pub fn creating_operation_object() {
        OperationObject {
            tags: Some(Vec::new()),
            summary: Some("summary".to_owned()),
            description: None,
            deprecated: None,
            parameters: None,
            request_body: None,
            operation_id: None,
        };
    }

    #[test]
    fn serializing_and_deserializing_media_type_object() {
        let operation_object = OperationObject {
            ..Default::default()
        };

        let serialized = serde_json::to_string(&operation_object).unwrap();
        let _: OperationObject = serde_json::from_str(&serialized).unwrap();
    }

    #[test]
    fn it_deserializes_parameter_refs() {
        let operation_object_str = r#"
parameters:
  - name: parameter
    in: query
  - $ref: valid ref
"#;
        let operation_object: OperationObject = serde_yaml::from_str(operation_object_str).unwrap();
        match operation_object.parameters {
            Some(parameters) => {
                let expected_num_of_parameters = 2;
                if parameters.len() != expected_num_of_parameters {
                    panic!("the number of parsed parameters doesn't match the number given. given: {}. parsed: {}", expected_num_of_parameters, parameters.len());
                }
            }
            None => panic!("operations parameters were not parsed at all")
        }
    }
}

