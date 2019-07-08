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


// value or ref
pub use common::value_or_ref::ValueOrRef;
// example
pub use definitions::example::example_object::ExampleObject;
pub use definitions::info::contact_info::SwaggerContactInfo;
// info
pub use definitions::info::info_object::InfoObject;
pub use definitions::info::licence_info::SwaggerLicenceInfo;
// media type
pub use definitions::media_type::media_type_object::MediaTypeObject;
// operation
pub use definitions::operation::operation_object::OperationObject;
// parameter
pub use definitions::parameter::parameter_object::ParameterObject;
pub use definitions::parameter::parameter_place::ParameterPlace;
pub use definitions::path::path_item::PathItem;
// definition
pub use definitions::path::paths::Paths;
// ref
pub use definitions::ref_::ref_::Ref;
// request
pub use definitions::request::request_body_object::RequestBodyObject;
// definition
pub use definitions::swagger_definition::SwaggerDefinition;

pub mod definitions;
pub mod common;

