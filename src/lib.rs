#[macro_use]
extern crate serde_derive;

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

#[cfg(test)]
mod tests {
    use crate::SwaggerDefinition;
    use crate::InfoObject;
    use std::collections::HashMap;

    #[test]
    fn create_swagger_definition() {
        let s = SwaggerDefinition {
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
