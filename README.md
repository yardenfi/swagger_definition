# Swagger Definition [![CircleCI](https://circleci.com/gh/yardenfi/swagger_definition/tree/master.svg?style=svg)](https://circleci.com/gh/yardenfi/swagger_definition/tree/master)
This crate is used to create a strong type implementation to the [openapi](https://swagger.io/specification/) specification in rust.
You could use it to create and validate the data that swagger ui needs to render all the paths, 
models and so on while steel using a rust api rather than serializing handwritten json.

Using [serde](https://github.com/serde-rs/serde) crate allows the user to serialize and deserialize from 
yaml, json and other data types the created data.
