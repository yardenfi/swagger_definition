use std::collections::HashMap;
use crate::definitions::media_type::media_type_object::MediaTypeObject;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequestBodyObject {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub content: HashMap<String, MediaTypeObject>
}


