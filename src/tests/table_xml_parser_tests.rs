use crate::{
    table_structs::{
        Action, AttributeType, Category, Classification, Column, CommonAttributeRules, CommonColumn, CommonSection, Condition, ConditionGroup, ConditionalFormatting, Control, DataType, DefaultStatus, Field, Formula, GridSpecific, Identifier, Level, Local, Matrix, MatrixField, MatrixSpecific, Metadata, Partition, Rule, Rules, Schema, Screen, ScreenGrid, Section, SelectOption, SpecificSection, Status, Suffix, Table
    },
    table_xml_parser::TableXmlParser,
};
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

#[test]
fn empty_file() {
    let table_xml_parser =
        TableXmlParser::read("./src/tests/inputs/MISSING_ELEMENT_TABLE.xml").unwrap();
    assert_eq!(table_xml_parser.errors[0].code, "MISSING_ELEMENT_TABLE");
}

#[test]
fn file_without_table_element() {
    let table_xml_parser =
        TableXmlParser::read("./src/tests/inputs/WRONG_FIRST_ELEMENT.xml").unwrap();
    assert_eq!(table_xml_parser.errors[0].code, "WRONG_FIRST_ELEMENT");
}

#[test]
fn file_with_empty_table_element() {
    let table_xml_parser =
        TableXmlParser::read("./src/tests/inputs/EMPTY_TABLE_ELEMENT.xml").unwrap();
    assert_eq!(table_xml_parser.errors[0].code, "EMPTY_TABLE_ELEMENT");
}

// #[test]
// fn process_large_file() {
//     let table_xml_parser = TableXmlParser::read("./src/tests/inputs/full-table.xml").unwrap();
//     assert!(true);
// }

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
            fields: vec![
                Field {
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
                },
                Field {
                    key: "COLOR".to_owned(),
                    level: "PRODUCT".to_owned(),
                    data_type: DataType::SingleSelect,
                    title: "Color".to_owned(),
                    description: Some("Product's color".to_owned()),
                    title_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Couleur".to_owned(),
                    }]),
                    description_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "La couleur du produit".to_owned(),
                    }]),
                    metadata: Some(vec![Metadata {
                        key: "SYSTEM".to_owned(),
                        value: "ERP".to_owned(),
                    }]),
                    prefix: None,
                    suffix: None,
                    precision: None,
                    suffixes: None,
                    options: Some(vec![
                        SelectOption {
                            key: "RED".to_owned(),
                            title: "Red".to_owned(),
                            color: Some("RED".to_owned()),
                            description: None,
                            title_locals: Some(vec![
                                Local { lang: "fra".to_owned(), value: "Rouge".to_owned()}
                            ]),
                            description_locals: None,
                            metadata: Some(vec![Metadata {
                                key: "SYSTEM".to_owned(),
                                value: "ERP".to_owned(),
                            }])
                        },
                        SelectOption {
                            key: "GREEN".to_owned(),
                            title: "Green".to_owned(),
                            color: Some("GREEN".to_owned()),
                            description: None,
                            title_locals: Some(vec![
                                Local { lang: "fra".to_owned(), value: "Vert".to_owned()}
                            ]),
                            description_locals: None,
                            metadata: None
                        }
                    ]),
                },
                Field {
                    key: "STORAGE_GB".to_owned(),
                    level: "PRODUCT".to_owned(),
                    data_type: DataType::Number,
                    title: "Storage GB".to_owned(),
                    description: Some("Storage capacity".to_owned()),
                    title_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Stockage GB".to_owned(),
                    }]),
                    description_locals: Some(vec![Local {
                        lang: "fra".to_owned(),
                        value: "Capacité de stockage".to_owned(),
                    }]),
                    metadata: Some(vec![Metadata {
                        key: "SYSTEM".to_owned(),
                        value: "ERP".to_owned(),
                    }]),
                    prefix: None,
                    suffix: Some("GB".to_owned()),
                    precision: None,
                    suffixes: None,
                    options: None,
                },
                Field {
                    key: "CAMERA_RESOLUTION".to_owned(),
                    level: "PRODUCT".to_owned(),
                    data_type: DataType::Number,
                    title: "Camera resolution".to_owned(),
                    description: Some("The resolution of an image is the number of pixels contained in the image per unit length.".to_owned()),
                    title_locals: None,
                    description_locals: None,
                    metadata: None,
                    prefix: None,
                    suffix: None,
                    precision: None,
                    suffixes: Some(vec![
                        Suffix {
                            key: "PIXEL".to_owned(),
                            title: "Pixel".to_owned(),
                            default: Some(true),
                            description: Some("A unit of count defining the number of pixels".to_owned()),
                            title_locals: None,
                            description_locals: None,
                            metadata: None
                        },
                        Suffix {
                            key: "MEGAPIXEL".to_owned(),
                            title: "Megapixel".to_owned(),
                            default: None,
                            description: Some("A unit of count equal to 10⁶ (1000000) pixels (picture elements).".to_owned()),
                            title_locals: None,
                            description_locals: None,
                            metadata: None
                        }
                    ]),
                    options: None,
                },
            ],
            formulas: vec![
                Formula {
                    attribute_type: AttributeType::Identifier,
                    key: "SUPPLIER-SUPPLIER_REF".to_owned(),
                    rules: vec![Rule {
                        priority: 1,
                        conditions: vec![ConditionGroup {
                            conditions: vec![
                                Condition::NotEmpty {
                                    source: "SUPPLIER".to_owned(),
                                },
                                Condition::NotEmpty {
                                    source: "SUPPLIER_REF".to_owned(),
                                },
                            ],
                        }],
                        action: Action::SetTextTemplate {
                            trim_spaces: true,
                            value: "{{source(\"SUPPLIER\",\"key\")}}-{{source(\"SUPPLIER_REF\")}}"
                                .to_owned(),
                        },
                    }],
                },
                Formula {
                    attribute_type: AttributeType::Field,
                    key: "VOLUME_CM_3".to_owned(),
                    rules: vec![
                        Rule {
                            priority: 1,
                            conditions: vec![
                                ConditionGroup {
                                    conditions: vec![
                                        Condition::NotEmpty { source: "WIDTH_CM".to_owned() },
                                        Condition::NotEmpty { source: "HEIGHT_CM".to_owned() },
                                        Condition::NotEmpty { source: "DEPTH_CM".to_owned() },
                                    ]
                                }
                            ],
                            action: Action::SetNumberTemplate { precision: 0, round: "CEILING".to_owned(), value: "{{source(\"WIDTH_CM\")}} * {{source(\"HEIGHT_CM\")}} * {{source(\"DEPTH_CM\")}}".to_owned() }
                        }
                    ]
                },
                Formula {
                    attribute_type: AttributeType::Field,
                    key: "DEEE_SCALE".to_owned(),
                    rules: vec![
                        Rule {
                            priority: 1,
                            conditions: vec![
                                ConditionGroup {
                                    conditions: vec![
                                        Condition::In {
                                            source: "SUPPLIER".to_owned(),
                                            values: vec![
                                                "SUPPLIER-1".to_owned(),
                                                "SUPPLIER-2".to_owned(),
                                            ]
                                        }
                                    ]
                                }
                            ],
                            action: Action::SetSelectableOptions {
                                values: vec![
                                    "DEEE-1".to_owned(),
                                    "DEEE-2".to_owned(),
                                ]
                            }
                        },
                        Rule {
                            priority: 2,
                            conditions: vec![
                                ConditionGroup {
                                    conditions: vec![
                                        Condition::In {
                                            source: "SUPPLIER".to_owned(),
                                            values: vec![
                                                "SUPPLIER-3".to_owned(),
                                            ]
                                        }
                                    ]
                                }
                            ],
                            action: Action::SetSelectableOptions {
                                values: vec![
                                    "DEEE-1".to_owned(),
                                    "DEEE-3".to_owned(),
                                ]
                            }
                        }
                    ]
                }
            ],
            matrix: Matrix {
                common: vec![
                    MatrixField { key: "TITLE_EN".to_owned() },
                    MatrixField { key: "COLOR".to_owned() },
                ],
                specifics: vec![
                    MatrixSpecific {
                        classification: "TYPOLOGY".to_owned(),
                        category: "PHONES".to_owned(),
                        fields: vec![
                            MatrixField { key: "STORAGE".to_owned() }
                        ]
                    }
                ],
            },
            conditional_formattings: vec![
                ConditionalFormatting {
                    key: "COMPLIANCE".to_owned(),
                    level: "PRODUCT".to_owned(),
                    title: "Compliance".to_owned(),
                    description: None,
                    title_locals: Some(vec![
                        Local { lang: "fra".to_owned(), value: "Conformité".to_owned()}
                    ]),
                    description_locals: None,
                    metadata: None,
                    default_status: DefaultStatus {
                        key: "VALID".to_owned(),
                        title: "Valid".to_owned(),
                        color: "GREEN".to_owned(),
                        description: None,
                        title_locals: Some(vec![
                            Local { lang: "fra".to_owned(), value: "Valide".to_owned()}
                        ]),
                        description_locals: None,
                        metadata: None
                    },
                    statuses: vec![
                        Status {
                            key: "INVALID".to_owned(),
                            title: "Invalid".to_owned(),
                            color: "RED".to_owned(),
                            priority: 1,
                            rules: Rules {
                                common: vec![
                                    CommonAttributeRules {
                                        attribute_type: AttributeType::Identifier,
                                        key: "EAN_13".to_owned(),
                                        controls: vec![
                                            Control::RuleBarcode {
                                                barcode_type: "EAN13".to_owned()
                                            },
                                            Control::RuleRequired
                                        ]
                                    },
                                    CommonAttributeRules {
                                        attribute_type: AttributeType::Field,
                                        key: "TITLE_EN".to_owned(),
                                        controls: vec![
                                            Control::RuleRequired,
                                            Control::RuleMinLength { min: 3 },
                                            Control::RuleMaxLength { max: 27 }
                                        ]
                                    },
                                    CommonAttributeRules {
                                        attribute_type: AttributeType::Field,
                                        key: "ATTR00480".to_owned(),
                                        controls: vec![
                                            Control::RuleRequired,
                                            Control::RuleCondition {
                                                key: "ATTR00480_NOT_EA".to_owned(),
                                                condition_groups: vec![
                                                    ConditionGroup {
                                                        conditions: vec![
                                                            Condition::Empty { source: "ATTR00480".to_owned() },
                                                            Condition::NotIn {
                                                                source: "ATTR00480".to_owned(),
                                                                values: vec![
                                                                    "PK (Pack / Carton/ Vendor outer pack)".to_owned(),
                                                                    "PL (Palette)".to_owned(),
                                                                ]
                                                            },
                                                            Condition::GreaterThan {
                                                                source: "ATTR00460".to_owned(),
                                                                value: 1
                                                            }
                                                        ]
                                                    }
                                                ],
                                                title: "L'arrondi de commande doit être PK ou PL si le multiple de commande est supérieur à 1".to_owned(),
                                                title_locals: Some(vec![
                                                    Local { lang: "fra".to_owned(), value: "Round must PK or PL".to_owned()}
                                                ])
                                            }
                                        ]
                                    }
                                ],
                                specifics: vec![]
                            },
                            description: None,
                            title_locals: None,
                            description_locals: None,
                            metadata: None
                        },
                        Status {
                            key: "RECOMMENDED".to_owned(),
                            title: "Recommended".to_owned(),
                            color: "ORANGE".to_owned(),
                            priority: 2,
                            description: None,
                            title_locals: None,
                            description_locals: None,
                            metadata: None,
                            rules: Rules {
                                common: vec![],
                                specifics: vec![]
                            }
                        }
                    ]
                }
            ],
            sections: vec![
                Section {
                    key: "IDENTIFIERS".to_owned(),
                    title: "Identifiers".to_owned(),
                    description: None,
                    title_locals: Some(vec![
                        Local { lang: "fra".to_owned(), value: "Identifiants".to_owned() }
                    ]),
                    description_locals: None,
                    metadata: None
                }
            ],
            screens: vec![
                Screen {
                    key: "ALL_PROPERTIES".to_owned(),
                    level: "PRODUCT".to_owned(),
                    title: "All properties".to_owned(),
                    position: 1,
                    description: None,
                    title_locals: Some(vec![
                        Local { lang: "fra".to_owned(), value: "Tous les attributs".to_owned()}
                    ]),
                    description_locals: None,
                    metadata: None,
                    grid: ScreenGrid {
                        line_height: "SHORT".to_owned(),
                        common: vec![
                            CommonSection {
                                key: "IDENTIFIERS".to_owned(),
                                position: 1,
                                columns: vec![
                                    CommonColumn::ColumnIdentifier(Column {
                                        key: "EAN_13".to_owned(),
                                        position: 1,
                                        fixed: Some(true),
                                        read_only: Some(true),
                                        width: None
                                    }), 
                                    CommonColumn::ColumnField(Column {
                                        key: "MAIN_IMAGE".to_owned(),
                                        position: 2,
                                        fixed: None,
                                        read_only: None,
                                        width: None
                                    }), 
                                    CommonColumn::ColumnClassification(Column {
                                        key: "TYPOLOGY".to_owned(),
                                        position: 3,
                                        fixed: None,
                                        read_only: None,
                                        width: None
                                    }), 
                                    CommonColumn::ColumnConditionalFormatting(Column {
                                        key: "COMPLIANCE".to_owned(),
                                        position: 4,
                                        fixed: None,
                                        read_only: None,
                                        width: None
                                    }), 
                                    CommonColumn::ColumnField(Column {
                                        key: "TITLE_EN".to_owned(),
                                        position: 5,
                                        fixed: None,
                                        read_only: None,
                                        width: None
                                    }), 
                                    CommonColumn::ColumnField(Column {
                                        key: "DESCRIPTION_EN".to_owned(),
                                        position: 6,
                                        fixed: None,
                                        read_only: None,
                                        width: None
                                    }), 
                                    CommonColumn::ColumnField(Column {
                                        key: "PRICE_EURO".to_owned(),
                                        position: 7,
                                        fixed: None,
                                        read_only: None,
                                        width: Some("SMALL".to_owned())
                                    }), 
                                ]
                            }
                        ],
                        specifics: vec![
                            GridSpecific {
                                classification: "TYPOLOGY".to_owned(),
                                category: "HOME_APPLIANCE".to_owned(),
                                sections: vec![
                                    SpecificSection {
                                        key: "INFORMATIONS".to_owned(),
                                        position: 1,
                                        columns: vec![
                                            Column {
                                                key: "INSTRUCTIONS".to_owned(),
                                                position: 1,
                                                width: None,
                                                fixed: None,
                                                read_only: None
                                            }
                                        ]
                                    }
                                ]
                            }
                        ]
                    }
                }
            ],
        },
    };
    println!("Starting parse xml");

    let table_xml_parser = TableXmlParser::read("./src/tests/inputs/valid.xml").unwrap();
    println!("{:#?}", table_xml_parser.warnings.len());
    println!("{:#?}", table_xml_parser.errors.len());
    // println!("{:#?}", table_xml_parser.table.description.unwrap());
    assert_eq!(table_xml_parser.table, expected);
}
