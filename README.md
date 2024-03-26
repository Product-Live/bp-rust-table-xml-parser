# Task import table schema

A rust implementation to read/write/validate the xml file expected in the Import Table Schema task of Data Factory.  
This implementation is used in Table Schema Editor project.

## Project structure

| Path                | Description                                      |
| ------------------- | ------------------------------------------------ |
| lib.rs              | Root file referencing all modules.               |
| table_structs.rs    | Structure definition (equivalent to types in TS) |
| table_xml_parser.rs | To read import_table_schema xml file             |
| table_xml_writer.rs | To write import_table_schema xml file            |
| table_validation.rs | All business rules validations                   |
| utils.rs            | Utilities functions                              |
| /tests/             | All tests                                        |

## Running tests

`cargo test`

## About the parser approach

Before this project, three methods have been tested:

| Method                                                                                         | Advantage                                                                         | Drawback                                  |
| ---------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------------------------- |
| use a lib that convert xml to struct, then write methods to parse                              | Fast to implement (until it get hard to read)                                     | Slow, high memory usage, hard to maintain |
| annotation on structures (write annotion on structure to define serialization/deserialization) | Fast to implement (until it get hard to read/write especially for serialization)  | Slow, high memory usage, hard to maintain |
| event parsing                                                                                  | Quickest method, low memory footprint, simpler to maintain on large xml structure |                                           |
