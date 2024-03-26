#[cfg(test)]
use pretty_assertions::assert_eq;

use crate::{table_structs::Partition, table_validation::{LogError, TableValidation}};

#[test]
fn whitespace_in_key() {
    let partition = Partition {
        key: "ACTIVE ".to_owned(),
        title: "Active".to_owned(),
        position: 1,
        description: None,
        title_locals: None,
        description_locals: None,
        metadata: None
    };
    let mut table_validation = TableValidation::new();
    table_validation.validate_partition(&partition, &mut vec![], &mut vec![], 0).unwrap();
    match &table_validation.errors[0] {
        LogError::Partition { code, message: _, xpath: _, partition_key: _key } => {
            assert_eq!(code.to_owned(), "KEY_STARTS_OR_ENDS_WHITESPACE".to_owned())
        },
        _ => assert!(false)
    }
}
