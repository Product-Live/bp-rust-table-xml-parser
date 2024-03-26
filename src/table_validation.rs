use std::{collections::HashSet, error::Error};

use crate::table_structs::{Partition, Table};

pub enum LogError {
    Partition {
        code: String,
        message: String,
        xpath: String,
        partition_key: String,
    },
    Classification {
        code: String,
        message: String,
        xpath: String,
        classification_key: String,
    },
    Category {
        code: String,
        message: String,
        xpath: String,
        classification_key: String,
        category_key: String,
    },
}

pub struct TableValidation {
    pub errors: Vec<LogError>,
}

impl TableValidation {
    pub fn new() -> Self {
        TableValidation {
            errors: vec![]
        }
    }
    pub fn validate(&mut self, table: &Table) -> Result<(), Box<dyn Error>> {
        self.validate_partitions(&table.schema.partitions)?;
        Ok(())
    }
    pub fn validate_partitions(
        &mut self,
        partitions: &Vec<Partition>,
    ) -> Result<(), Box<dyn Error>> {
        let mut partition_keys: Vec<String> = vec![];
        let mut partition_indexes: Vec<(String, usize)> = vec![];
        for (idx, partition) in partitions.iter().enumerate() {
            self.validate_partition(partition, &mut partition_keys, &mut partition_indexes, idx)?;
        }
        match control_text_uniqueness(partition_keys)? {
            Some(keys) => {
                for key in keys.iter() {
                    self.errors.push(LogError::Partition {
                        code: "DUPLICATE_KEY".to_owned(),
                        message: "Partition' attribute @key is not unique.".to_owned(),
                        xpath: format!(
                            "/Table/Schema/Partitions/Partition[@key='{}']/@key",
                            key
                        ),
                        partition_key: key.to_owned(),
                    })
                }
            }
            None => (),
        };
        match control_number_uniqueness(partition_indexes)? {
            Some(keys) => {
                for key in keys.iter() {
                    self.errors.push(LogError::Partition {
                        code: "DUPLICATE_INDEX".to_owned(),
                        message: "Partition' attribute @index is not unique.".to_owned(),
                        xpath: format!(
                            "/Table/Schema/Partitions/Partition[@key='{}']/@index",
                            key
                        ),
                        partition_key: key.to_owned(),
                    })
                }
            }
            None => (),
        };
        Ok(())
    }
    pub fn validate_partition(&mut self, partition: &Partition, partition_keys: &mut Vec<String>, partition_indexes: &mut Vec<(String, usize)>, idx: usize) -> Result<(), Box<dyn Error>> {
        let key = &partition.key;
        partition_keys.push(key.to_owned());
        partition_indexes.push((key.to_owned(), idx));
        if key.ends_with(" ") || key.starts_with(" ") {
            self.errors.push(LogError::Partition {
                code: "KEY_STARTS_OR_ENDS_WHITESPACE".to_owned(),
                message: format!(
                    "Partition attribute @key starts or ends with a whitespace."
                ),
                xpath: format!("/Table/Schema/Partitions/Partition[@key='{}']/@key", key),
                partition_key: key.to_owned(),
            })
        }
        match control_min_length(partition.key.to_owned(), 0)? {
            Some(value_length) => self.errors.push(LogError::Partition {
                code: "MIN_LENGTH".to_owned(),
                message: format!(
                "Partition attribute @key is not greater than {} characters (actual: {}).",
                0, value_length
            ),
                xpath: format!("/Table/Schema/Partitions/Partition[@key='{}']/@key", key),
                partition_key: key.to_owned(),
            }),
            None => (),
        }
        match control_max_length(partition.key.to_owned(), 255)? {
            Some(value_length) => self.errors.push(LogError::Partition {
                code: "MAX_LENGTH".to_owned(),
                message: format!(
                "Partition attribute @key is not lower than {} characters (actual: {}).",
                255, value_length
            ),
                xpath: format!("/Table/Schema/Partitions/Partition[@key='{}']/@key", key),
                partition_key: key.to_owned(),
            }),
            None => (),
        }
        match control_min_length(partition.title.to_owned(), 0)? {
            Some(value_length) => self.errors.push(LogError::Partition {
                code: "MIN_LENGTH".to_owned(),
                message: format!(
                "Partition element Title is not greater than {} characters (actual: {}).",
                0, value_length
            ),
                xpath: format!("/Table/Schema/Partitions/Partition[@key='{}']/Title", key),
                partition_key: key.to_owned(),
            }),
            None => (),
        }
        match control_max_length(partition.title.to_owned(), 255)? {
            Some(value_length) => self.errors.push(LogError::Partition {
                code: "MAX_LENGTH".to_owned(),
                message: format!(
                    "Partition element Title is not lower than {} characters (actual: {}).",
                    255, value_length
                ),
                xpath: format!("/Table/Schema/Partitions/Partition[@key='{}']/Title", key),
                partition_key: key.to_owned(),
            }),
            None => (),
        }
        Ok(())
    } 
}

// Utils
fn control_text_uniqueness(elements: Vec<String>) -> Result<Option<Vec<String>>, Box<dyn Error>> {
    let unique_elements: HashSet<String> = elements.to_vec().into_iter().collect();
    let mut duplicate_keys: Vec<String> = vec![];
    for key in unique_elements {
        let count = elements.iter().filter(|k| k.as_str() == key).count();
        if count > 1 {
            duplicate_keys.push(key);
        }
    }
    if duplicate_keys.len() > 0 {
        Ok(Some(duplicate_keys))
    } else {
        Ok(None)
    }
}
fn control_number_uniqueness(
    elements: Vec<(String, usize)>,
) -> Result<Option<Vec<String>>, Box<dyn Error>> {
    let unique_indexes: HashSet<usize> = elements.iter().map(|element| element.1).collect();
    let mut duplicate_index_keys: Vec<String> = vec![];
    for (idx, index) in unique_indexes.iter().enumerate() {
        let count = elements.iter().filter(|k| k.1 == *index).count();
        if count > 1 {
            duplicate_index_keys.push(elements[idx].0.to_owned());
        }
    }
    if duplicate_index_keys.len() > 0 {
        Ok(Some(duplicate_index_keys))
    } else {
        Ok(None)
    }
}
// fn control_position_uniqueness(&mut self, elements: Vec<usize>, label: &str, xpath: String, path: Vec<String>) {
//     let unique_elements: HashSet<usize> = elements.to_vec().into_iter().collect();
//     for index in unique_elements {
//         let indices = elements
//             .iter()
//             .enumerate()
//             .filter(|(_, &i)| i == index)
//             .map(|(indice, _)| indice)
//             .collect::<Vec<_>>();
//         if &indices.len() > &1 {
//             self.add_log(Log::ArrayElement {
//                 core: LogCore {
//                     code: "G-2".to_owned(),
//                     label: format!("{}' position is not unique.", label),
//                     xpath: format!("/Table/Schema/{}[@position='{}']", xpath, index),
//                     path: path.to_owned(),
//                 },
//                 indexes: indices,
//             })
//         }
//     }
// }
fn control_min_length(value: String, min: usize) -> Result<Option<usize>, Box<dyn Error>> {
    let value_length = value.len();
    if value_length < min {
        Ok(Some(value_length))
    } else {
        Ok(None)
    }
}
fn control_max_length(value: String, max: usize) -> Result<Option<usize>, Box<dyn Error>> {
    let value_length = value.len();
    if value_length >= max {
        Ok(Some(value_length))
    } else {
        Ok(None)
    }
}
fn control_starts_or_ends_whitespace(value: String) -> Result<bool, Box<dyn Error>> {
    if value.ends_with(" ") || value.starts_with(" ") {
        Ok(true)
    } else {
        Ok(false)
    }
}
