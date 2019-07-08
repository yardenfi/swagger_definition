use std::fs;
use std::path::{Path, PathBuf};

pub fn load_pet_store() -> String {
    let path = &PathBuf::from("./tests/common/assets/petstore.yaml");
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}