//! # Swagger Definition [![CircleCI](https://circleci.com/gh/yardenfi/swagger_definition/tree/master.svg?style=svg)](https://circleci.com/gh/yardenfi/swagger_definition/tree/master)
//! This crate is used to create a strong type implementation to the [openapi](https://swagger.io/specification/) specification in rust.
//! You could use it to create and validate the data that swagger ui needs to render all the paths,
//! models and so on while steel using a rust api rather than serializing handwritten json.
//!
//! Using [serde](https://github.com/serde-rs/serde) crate allows the user to serialize and deserialize from
//! yaml, json and other data types the created data.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


pub mod definitions;

// definition
pub use definitions::swagger_definition::SwaggerDefinition;

// info
pub use definitions::info::info_object::InfoObject;
pub use definitions::info::licence_info::SwaggerLicenceInfo;
pub use definitions::info::contact_info::SwaggerContactInfo;

// operation
pub use definitions::operation::operation_object::OperationObject;

// parameter
pub use definitions::parameter::parameter_object::ParameterObject;
pub use definitions::parameter::parameter_place::ParameterPlace;

// definition
pub use definitions::path::paths::Paths;
pub use definitions::path::path_item::PathItem;

// request
pub use definitions::request::request_body_object;

// media type
pub use definitions::media_type::media_type_object;

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
