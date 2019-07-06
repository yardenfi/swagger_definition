use crate::models::swagger_definition::swagger_info::SwaggerInfo;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaggerDefinition {
    pub swagger: String,
    pub info: SwaggerInfo,
    pub paths: Paths,
    pub host: Option<String>,
    pub schemas: Option<Vec<String>>,
}


pub mod swagger_info {
    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct SwaggerInfo {
        pub version: String,
        pub title: String,
        pub description: Option<String>,
        #[serde(rename = "termsOfService")]
        pub terms_of_service: Option<String>,
        pub contact: Option<SwaggerContactInfo>,
        pub license: Option<SwaggerLiscenceInfo>
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct SwaggerContactInfo {
        pub email: Option<String>
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct SwaggerLiscenceInfo {
        pub name: Option<String>,
        pub url: Option<String>
    }
}

pub type Paths = HashMap<String, PathItem>;

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParameterObject {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: ParameterPlace,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParameterPlace {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "cookie")]
    Cookie
}

impl Default for ParameterPlace {
    fn default() -> Self {
        return ParameterPlace::Query;
    }
}
