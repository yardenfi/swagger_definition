use swagger_definition::{SwaggerContactInfo, SwaggerDefinition};
use std::fs;
use crate::common::load_examples::load_pet_store;

mod common;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


#[test]
fn it_deserialize_yaml_and_serialize_into_json() {
    let pet_store_yaml_str = load_pet_store();
    let definition: SwaggerDefinition = serde_yaml::from_str(&pet_store_yaml_str).expect("to be successful");
    serde_json::to_string(&definition);
}
