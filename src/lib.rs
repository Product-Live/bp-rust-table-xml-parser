mod table_structs;
mod table_validation;
mod table_xml_parser;
mod table_xml_writer;
mod utils;

#[cfg(test)]
mod tests {
    use crate::table_structs::{
        Category, Classification, DataType, Field, Identifier, Level, Local, Matrix, Metadata,
        Partition, Schema, Table,
    };
    use crate::table_xml_parser::{self, TableXmlParser};
    use crate::table_xml_writer::TableXmlWriter;

    #[test]
    fn empty_file() {
        let table_xml_parser = TableXmlParser::read("./tests/MISSING_ELEMENT_TABLE.xml").unwrap();
        assert_eq!(table_xml_parser.errors[0].code, "MISSING_ELEMENT_TABLE");
    }

    #[test]
    fn file_without_table_element() {
        let table_xml_parser = TableXmlParser::read("./tests/WRONG_FIRST_ELEMENT.xml").unwrap();
        assert_eq!(table_xml_parser.errors[0].code, "WRONG_FIRST_ELEMENT");
    }

    #[test]
    fn file_with_empty_table_element() {
        let table_xml_parser = TableXmlParser::read("./tests/EMPTY_TABLE_ELEMENT.xml").unwrap();
        assert_eq!(table_xml_parser.errors[0].code, "EMPTY_TABLE_ELEMENT");
    }

    #[test]
    fn xml_parser() {
        let expected = Table {
            key: "PRODUCTS".to_owned(),
            title: "Products".to_owned(),
            position: 1,
            color: "BLUE".to_owned(),
            description: Some("Products' table".to_owned()),
            title_locals: Some(vec![Local {
                lang: "fra".to_owned(),
                value: "Produits".to_owned(),
            }]),
            description_locals: Some(vec![Local {
                lang: "fra".to_owned(),
                value: "La table produits".to_owned(),
            }]),
            schema: Schema {
                partitions: vec![Partition {
                    key: "ACTIVE".to_owned(),
                    title: "Active".to_owned(),
                    position: 1,
                    description: None,
                    title_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Actifs".to_owned(),
                    }]),
                    description_locals: None,
                    metadata: None,
                }],
                levels: vec![Level {
                    key: "PRODUCT".to_owned(),
                    title: "Product".to_owned(),
                    index: 1,
                    description: None,
                    title_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Produit".to_owned(),
                    }]),
                    description_locals: None,
                    metadata: None,
                }],
                identifiers: vec![Identifier {
                    key: "EAN_13".to_owned(),
                    title: "EAN 13".to_owned(),
                    index: 1,
                    level: "PRODUCT".to_owned(),
                    description: None,
                    title_locals: None,
                    description_locals: None,
                    metadata: Some(vec![Metadata {
                        key: "SYSTEM".to_owned(),
                        value: "ERP".to_owned(),
                    }]),
                }],
                classifications: vec![Classification {
                    key: "TYPOLOGY".to_owned(),
                    title: "Typology".to_owned(),
                    description: None,
                    title_locals: None,
                    description_locals: None,
                    metadata: None,
                    categories: vec![
                        Category {
                            key: "HOME_APPLIANCE".to_owned(),
                            parent: None,
                            title: "Home appliance".to_owned(),
                            description: None,
                            title_locals: None,
                            description_locals: None,
                            metadata: None,
                        },
                        Category {
                            key: "PHONES".to_owned(),
                            parent: Some("HOME_APPLIANCE".to_owned()),
                            title: "Phones".to_owned(),
                            description: None,
                            title_locals: None,
                            description_locals: None,
                            metadata: None,
                        },
                    ],
                }],
                fields: vec![Field {
                    key: "TITLE_EN".to_owned(),
                    level: "PRODUCT".to_owned(),
                    data_type: DataType::SingleLineText,
                    title: "Title EN".to_owned(),
                    description: Some("Product's title in english".to_owned()),
                    title_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Titre EN".to_owned(),
                    }]),
                    description_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Le titre du produit en anglais".to_owned(),
                    }]),
                    metadata: Some(vec![Metadata {
                        key: "SYSTEM".to_owned(),
                        value: "ERP".to_owned(),
                    }]),
                    prefix: None,
                    suffix: None,
                    precision: None,
                    suffixes: None,
                    options: None,
                }],
                formulas: vec![],
                matrix: Matrix {
                    common: vec![],
                    specifics: vec![],
                },
                conditional_formattings: vec![],
                sections: vec![],
                screens: vec![],
            },
        };
        println!("Starting parse xml");

        match TableXmlParser::read("./tests/table.xml") {
            Ok(table_xml_parser) => {
                println!("File parsed");
                println!("{:#?}", table_xml_parser.warnings.len());
                println!("{:#?}", table_xml_parser.errors.len());
                println!("{:#?}", table_xml_parser.table.schema.fields.len());
                // assert_eq!(table, expected)
                assert!(false)
            }
            Err(errors) => {
                println!("File parsed");
                println!("{:#?}", errors);
                assert!(false)
            }
        }
    }

    // #[test]
    // fn xml_writer() {
    //     match TableXmlParser::read("./tests/input.xml") {
    //         Ok(table) => {
    //             match TableXmlWriter::write(&table, "./tests/output.xml") {
    //                 Ok(_) => assert!(true),
    //                 Err(errors) => {
    //                     println!("{:#?}", errors);
    //                     assert!(false)
    //                 }
    //             }
    //             assert!(true);
    //         }
    //         Err(errors) => {
    //             println!("{:#?}", errors);
    //             assert!(false)
    //         }
    //     }
    // }
}
