pub mod partitions {
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use crate::{
        table_structs::Partition,
        table_validation::{LogError, TableValidation},
    };
    #[test]
    fn whitespace_in_key() {
        let partition = Partition {
            key: "ACTIVE ".to_owned(),
            title: "Active".to_owned(),
            position: 1,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_partition(&partition).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "KEY_STARTS_OR_ENDS_WHITESPACE".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn min_length_key() {
        let partition = Partition {
            key: "".to_owned(),
            title: "Active".to_owned(),
            position: 1,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_partition(&partition).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MIN_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn max_length_key() {
        let partition = Partition {
            key: "An extra long key with lorem Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo conseq".to_owned(),
            title: "Active".to_owned(),
            position: 1,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_partition(&partition).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MAX_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn min_length_title() {
        let partition = Partition {
            key: "ACTIVE".to_owned(),
            title: "".to_owned(),
            position: 1,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_partition(&partition).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MIN_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn max_length_title() {
        let partition = Partition {
            key: "ACTIVE".to_owned(),
            title: "An extra long key with lorem Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo conseq".to_owned(),
            position: 1,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_partition(&partition).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MAX_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn key_uniqueness() {
        let partitions = vec![
            Partition {
                key: "ACTIVE".to_owned(),
                title: "Actives".to_owned(),
                position: 1,
                description: None,
                title_locals: None,
                description_locals: None,
                metadata: None,
            },
            Partition {
                key: "ACTIVE".to_owned(),
                title: "Actives".to_owned(),
                position: 2,
                description: None,
                title_locals: None,
                description_locals: None,
                metadata: None,
            },
        ];
        let mut table_validation = TableValidation::new();
        table_validation.validate_partitions(&partitions).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "DUPLICATE_KEY".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn position_uniqueness() {
        let partitions = vec![
            Partition {
                key: "ACTIVE".to_owned(),
                title: "Actives".to_owned(),
                position: 1,
                description: None,
                title_locals: None,
                description_locals: None,
                metadata: None,
            },
            Partition {
                key: "ARCHIVED".to_owned(),
                title: "Archived".to_owned(),
                position: 1,
                description: None,
                title_locals: None,
                description_locals: None,
                metadata: None,
            },
        ];
        let mut table_validation = TableValidation::new();
        table_validation.validate_partitions(&partitions).unwrap();
        match &table_validation.errors[0] {
            LogError::Partition {
                code,
                message: _,
                xpath: _,
                partition_key: _key,
            } => {
                assert_eq!(code.to_owned(), "DUPLICATE_INDEX".to_owned())
            }
            _ => assert!(false),
        }
    }
}

pub mod fields {
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use crate::{
        table_structs::{DataType, Field},
        table_validation::{LogError, TableValidation},
    };
    #[test]
    fn whitespace_in_key() {
        let field = Field {
            key: "TITLE  ".to_owned(),
            title: "Title".to_owned(),
            data_type: DataType::SingleLineText,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            level: "PRODUCT".to_owned(),
            prefix: None,
            suffix: None,
            precision: None,
            suffixes: None,
            options: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_field(&field).unwrap();
        match &table_validation.errors[0] {
            LogError::Field {
                code,
                message: _,
                xpath: _,
                field_key: _key,
            } => {
                assert_eq!(code.to_owned(), "KEY_STARTS_OR_ENDS_WHITESPACE".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn min_length_key() {
        let field = Field {
            key: "T".to_owned(),
            title: "Title".to_owned(),
            data_type: DataType::SingleLineText,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            level: "PRODUCT".to_owned(),
            prefix: None,
            suffix: None,
            precision: None,
            suffixes: None,
            options: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_field(&field).unwrap();
        match &table_validation.errors[0] {
            LogError::Field {
                code,
                message: _,
                xpath: _,
                field_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MIN_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn max_length_key() {
        let field = Field {
            key: "An extra long key with lorem Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo conseq".to_owned(),
            title: "Title".to_owned(),
            data_type: DataType::SingleLineText,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            level: "PRODUCT".to_owned(),
            prefix: None,
            suffix: None,
            precision: None,
            suffixes: None,
            options: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_field(&field).unwrap();
        match &table_validation.errors[0] {
            LogError::Field {
                code,
                message: _,
                xpath: _,
                field_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MAX_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn min_length_title() {
        let field = Field {
            key: "Title".to_owned(),
            title: "t".to_owned(),
            data_type: DataType::SingleLineText,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            level: "PRODUCT".to_owned(),
            prefix: None,
            suffix: None,
            precision: None,
            suffixes: None,
            options: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_field(&field).unwrap();
        match &table_validation.errors[0] {
            LogError::Field {
                code,
                message: _,
                xpath: _,
                field_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MIN_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn max_length_title() {
        let field = Field {
            key: "TITLE".to_owned(),
            title: "An extra long key with lorem Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo conseq".to_owned(),
            data_type: DataType::SingleLineText,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            level: "PRODUCT".to_owned(),
            prefix: None,
            suffix: None,
            precision: None,
            suffixes: None,
            options: None,
        };
        let mut table_validation = TableValidation::new();
        table_validation.validate_field(&field).unwrap();
        match &table_validation.errors[0] {
            LogError::Field {
                code,
                message: _,
                xpath: _,
                field_key: _key,
            } => {
                assert_eq!(code.to_owned(), "MAX_LENGTH".to_owned())
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn key_uniqueness() {
        let fields = vec![
            Field {
                key: "TITLE".to_owned(),
                title: "Title".to_owned(),
                data_type: DataType::SingleLineText,
                description: None,
                title_locals: None,
                description_locals: None,
                metadata: None,
                level: "PRODUCT".to_owned(),
                prefix: None,
                suffix: None,
                precision: None,
                suffixes: None,
                options: None,
            },
            Field {
                key: "TITLE".to_owned(),
                title: "Title".to_owned(),
                data_type: DataType::SingleLineText,
                description: None,
                title_locals: None,
                description_locals: None,
                metadata: None,
                level: "PRODUCT".to_owned(),
                prefix: None,
                suffix: None,
                precision: None,
                suffixes: None,
                options: None,
            },
        ];
        let mut table_validation = TableValidation::new();
        table_validation.validate_fields(&fields).unwrap();
        match &table_validation.errors[0] {
            LogError::Field {
                code,
                message: _,
                xpath: _,
                field_key: _key,
            } => {
                assert_eq!(code.to_owned(), "DUPLICATE_KEY".to_owned())
            }
            _ => assert!(false),
        }
    }
}
