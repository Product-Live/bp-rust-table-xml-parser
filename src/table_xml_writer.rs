use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
// use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use quick_xml::Error;


use crate::table_structs::Table;

pub struct TableXmlWriter {}

impl TableXmlWriter {
    pub fn write(table: &Table, path: &str) -> Result<(), Error> {
        // let mut buffer = File::create(path)?;
        let mut writer = Writer::new(BufWriter::new(File::create(path).unwrap()));

        writer
            .create_element("Table")
            .with_attribute(("key", table.key.to_string().as_str()))
            .write_inner_content::<_, Error>(|writer| {
                writer
                    .create_element("Title")
                    .write_text_content(BytesText::new(table.title.to_string().as_str()))?;
                Ok(())
            })?;
        Ok(())
    }
    pub fn save(&self, path: &str) {
        let mut writer = Writer::new(BufWriter::new(File::create(path).unwrap()));
        let _ = writer
            .create_element("Table")
            .with_attribute(("key", self.key.to_string().as_str()))
            .write_inner_content(|writer| {
                let _ = writer
                    .create_element("Title")
                    .write_text_content(BytesText::new(self.title.to_string().as_str()))?;
                let _ = writer
                    .create_element("Color")
                    .write_text_content(BytesText::new(self.color.to_string().as_str()))?;
                let _ = writer
                    .create_element("Position")
                    .write_text_content(BytesText::new(self.position.to_string().as_str()))?;
                match &self.description {
                    None => {},
                    Some(description) => {
                        let _ = writer
                            .create_element("Description")
                            .write_text_content(BytesText::new(description.to_string().as_str()));
                    }
                }
                match &self.title_locals {
                    None => {},
                    Some(locals) => {
                        for local in locals.iter() {
                            let _ = writer
                                .create_element("Title-Local")
                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                        }
                    }
                }
                match &self.description_locals {
                    None => {},
                    Some(locals) => {
                        for local in locals.iter() {
                            let _ = writer
                                .create_element("Description-Local")
                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                        }
                    }
                }
                let _ = writer
                    .create_element("Schema")
                    .write_inner_content(|writer| {
                        let _ = writer
                            .create_element("Levels")
                            .write_inner_content(|writer| {
                                for level in self.schema.levels.iter() {
                                    let _ = writer
                                        .create_element("Level")
                                        .with_attribute(("key", level.key.to_string().as_str()))
                                        .with_attribute(("index", level.index.to_string().as_str()))
                                        .write_inner_content(|writer| {
                                            let _ = writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(level.title.to_string().as_str()))?;
                                            match &level.title_locals {
                                                None => {},
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        let _ = writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_string().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                    }
                                                }
                                            }
                                            Ok(())
                                        });
                                }
                                Ok(())
                            });
                            let _ = writer
                                .create_element("Partitions")
                                .write_inner_content(|writer| {
                                    for partition in self.schema.partitions.iter() {
                                        let _ = writer
                                            .create_element("Partition")
                                            .with_attribute(("key", partition.key.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                let _ = writer
                                                    .create_element("Title")
                                                    .write_text_content(BytesText::new(partition.title.to_string().as_str()))?;
                                                let _ = writer
                                                    .create_element("Position")
                                                    .write_text_content(BytesText::new(partition.position.to_string().as_str()))?;
                                                match &partition.title_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Title-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &partition.description_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Description-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &partition.metadata {
                                                    None => {},
                                                    Some(metadata) => {
                                                        for meta in metadata.iter() {
                                                            let _ = writer
                                                                .create_element("Metadata")
                                                                .with_attribute(("key", meta.key.to_string().as_str()))
                                                                .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                            let _ = writer
                                .create_element("Identifiers")
                                .write_inner_content(|writer| {
                                    for identifier in self.schema.identifiers.iter() {
                                        let _ = writer
                                            .create_element("Identifier")
                                            .with_attribute(("key", identifier.key.to_string().as_str()))
                                            .with_attribute(("index", identifier.index.to_string().as_str()))
                                            .with_attribute(("level", identifier.level.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                let _ = writer
                                                    .create_element("Title")
                                                    .write_text_content(BytesText::new(identifier.title.to_string().as_str()))?;
                                                match &identifier.title_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Title-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &identifier.description_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Description-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &identifier.metadata {
                                                    None => {},
                                                    Some(metadata) => {
                                                        for meta in metadata.iter() {
                                                            let _ = writer
                                                                .create_element("Metadata")
                                                                .with_attribute(("key", meta.key.to_string().as_str()))
                                                                .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                            let _ = writer
                                .create_element("Classifications")
                                .write_inner_content(|writer| {
                                    for classification in self.schema.classifications.iter() {
                                        let _ = writer
                                            .create_element("Classification")
                                            .with_attribute(("key", classification.key.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                let _ = writer
                                                    .create_element("Title")
                                                    .write_text_content(BytesText::new(classification.title.to_string().as_str()))?;
                                                match &classification.title_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Title-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &classification.description_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Description-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &classification.metadata {
                                                    None => {},
                                                    Some(metadata) => {
                                                        for meta in metadata.iter() {
                                                            let _ = writer
                                                                .create_element("Metadata")
                                                                .with_attribute(("key", meta.key.to_string().as_str()))
                                                                .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &classification.categories {
                                                    None => {},
                                                    Some(categories) => {
                                                        let _ = writer
                                                            .create_element("Categories")
                                                            .write_inner_content(|writer| {
                                                                for category in categories.iter() {
                                                                    let _ = writer
                                                                        .create_element("Category")
                                                                        .with_attribute(("key", category.key.to_string().as_str()))
                                                                        .write_inner_content(|writer| {
                                                                            let _ = writer
                                                                                .create_element("Title")
                                                                                .write_text_content(BytesText::new(category.title.to_string().as_str()))?;
                                                                            match &category.title_locals {
                                                                                None => {},
                                                                                Some(locals) => {
                                                                                    for local in locals.iter() {
                                                                                        let _ = writer
                                                                                            .create_element("Title-Local")
                                                                                            .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                            .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match &category.description_locals {
                                                                                None => {},
                                                                                Some(locals) => {
                                                                                    for local in locals.iter() {
                                                                                        let _ = writer
                                                                                            .create_element("Description-Local")
                                                                                            .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                            .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match &category.metadata {
                                                                                None => {},
                                                                                Some(metadata) => {
                                                                                    for meta in metadata.iter() {
                                                                                        let _ = writer
                                                                                            .create_element("Metadata")
                                                                                            .with_attribute(("key", meta.key.to_string().as_str()))
                                                                                            .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                                                    }
                                                                                }
                                                                            }
                                                                            Ok(())
                                                                        });
                                                                }
                                                                Ok(())
                                                            });
                                                    }
                                                }
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                            let _ = writer
                                .create_element("Fields")
                                .write_inner_content(|writer| {
                                    for field in self.schema.fields.iter() {
                                        let _ = writer
                                            .create_element("Field")
                                            .with_attribute(("key", field.key.to_string().as_str()))
                                            .with_attribute(("type", field.data_type.to_string().as_str()))
                                            .with_attribute(("level", field.level.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                let _ = writer
                                                    .create_element("Title")
                                                    .write_text_content(BytesText::new(field.title.to_string().as_str()))?;
                                                match &field.title_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Title-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &field.description_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Description-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &field.metadata {
                                                    None => {},
                                                    Some(metadata) => {
                                                        for meta in metadata.iter() {
                                                            let _ = writer
                                                                .create_element("Metadata")
                                                                .with_attribute(("key", meta.key.to_string().as_str()))
                                                                .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &field.suffix {
                                                    None => {},
                                                    Some(suffix) => {
                                                        let _ = writer
                                                            .create_element("Suffix")
                                                            .write_text_content(BytesText::new(suffix.to_string().as_str()));
                                                    }
                                                }
                                                match &field.prefix {
                                                    None => {},
                                                    Some(prefix) => {
                                                        let _ = writer
                                                            .create_element("Prefix")
                                                            .write_text_content(BytesText::new(prefix.to_string().as_str()));
                                                    }
                                                }
                                                match &field.precision {
                                                    None => {},
                                                    Some(precision) => {
                                                        let _ = writer
                                                            .create_element("Precision")
                                                            .write_text_content(BytesText::new(precision.to_string().as_str()));
                                                    }
                                                }
                                                match &field.suffixes {
                                                    None => {},
                                                    Some(suffixes) => {
                                                        let _ = writer
                                                            .create_element("Suffixes")
                                                            .write_inner_content(|writer| {
                                                                for suffix in suffixes.iter() {
                                                                    match &suffix.default {
                                                                        None => {
                                                                            let _ = writer
                                                                                .create_element("Suffix")
                                                                                .with_attribute(("key", suffix.key.to_string().as_str()))
                                                                                .write_inner_content(|writer| {
                                                                                    let _ = writer
                                                                                        .create_element("Title")
                                                                                        .write_text_content(BytesText::new(suffix.title.to_string().as_str()))?;
                                                                                    match &suffix.title_locals {
                                                                                        None => {},
                                                                                        Some(locals) => {
                                                                                            for local in locals.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Title-Local")
                                                                                                    .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Ok(())
                                                                                });
                                                                        },
                                                                        Some(default) => {
                                                                            let _ = writer
                                                                                .create_element("Suffix")
                                                                                .with_attribute(("key", suffix.key.to_string().as_str()))
                                                                                .with_attribute(("default", default.to_string().as_str()))
                                                                                .write_inner_content(|writer| {
                                                                                    let _ = writer
                                                                                        .create_element("Title")
                                                                                        .write_text_content(BytesText::new(suffix.title.to_string().as_str()))?;
                                                                                    match &suffix.title_locals {
                                                                                        None => {},
                                                                                        Some(locals) => {
                                                                                            for local in locals.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Title-Local")
                                                                                                    .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Ok(())
                                                                                });
                                                                        }
                                                                    }
                                                                }
                                                                Ok(())
                                                            });
                                                    }
                                                }
                                                match &field.options {
                                                    None => {},
                                                    Some(options) => {
                                                        let _ = writer
                                                            .create_element("Options")
                                                            .write_inner_content(|writer| {
                                                                for option in options.iter() {
                                                                    match &option.color {
                                                                        None => {
                                                                            let _ = writer
                                                                                .create_element("Option")
                                                                                .with_attribute(("key", option.key.to_string().as_str()))
                                                                                .write_inner_content(|writer| {
                                                                                    let _ = writer
                                                                                        .create_element("Title")
                                                                                        .write_text_content(BytesText::new(option.title.to_string().as_str()))?;
                                                                                    match &option.title_locals {
                                                                                        None => {},
                                                                                        Some(locals) => {
                                                                                            for local in locals.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Title-Local")
                                                                                                    .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    match &option.description_locals {
                                                                                        None => {},
                                                                                        Some(locals) => {
                                                                                            for local in locals.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Description-Local")
                                                                                                    .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    match &option.metadata {
                                                                                        None => {},
                                                                                        Some(metadata) => {
                                                                                            for meta in metadata.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Metadata")
                                                                                                    .with_attribute(("key", meta.key.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Ok(())
                                                                                });
                                                                        },
                                                                        Some(color) => {
                                                                            let _ = writer
                                                                                .create_element("Option")
                                                                                .with_attribute(("key", option.key.to_string().as_str()))
                                                                                .with_attribute(("color", color.to_string().as_str()))
                                                                                .write_inner_content(|writer| {
                                                                                    let _ = writer
                                                                                        .create_element("Title")
                                                                                        .write_text_content(BytesText::new(option.title.to_string().as_str()))?;
                                                                                    match &option.title_locals {
                                                                                        None => {},
                                                                                        Some(locals) => {
                                                                                            for local in locals.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Title-Local")
                                                                                                    .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    match &option.description_locals {
                                                                                        None => {},
                                                                                        Some(locals) => {
                                                                                            for local in locals.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Description-Local")
                                                                                                    .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    match &option.metadata {
                                                                                        None => {},
                                                                                        Some(metadata) => {
                                                                                            for meta in metadata.iter() {
                                                                                                let _ = writer
                                                                                                    .create_element("Metadata")
                                                                                                    .with_attribute(("key", meta.key.to_string().as_str()))
                                                                                                    .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Ok(())
                                                                                });
                                                                        }
                                                                    }
                                                                }
                                                                Ok(())
                                                            });
                                                    }
                                                }
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                            let _ = writer
                                .create_element("Matrix")
                                .write_inner_content(|writer| {
                                    let _ = writer
                                        .create_element("Common")
                                        .write_inner_content(|writer| {
                                            for field in self.schema.matrix.common.iter() {
                                                let _ = writer
                                                    .create_element("Field")
                                                    .with_attribute(("key", field.key.to_string().as_str()))
                                                    .write_empty();
                                            }
                                            Ok(())
                                        });
                                    for specific in self.schema.matrix.specifics.iter() {
                                        let _ = writer
                                            .create_element("Specific")
                                            .with_attribute(("classification", specific.classification.to_string().as_str()))
                                            .with_attribute(("category", specific.category.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                for field in specific.fields.iter() {
                                                    let _ = writer
                                                        .create_element("Field")
                                                        .with_attribute(("key", field.key.to_string().as_str()))
                                                        .write_empty();
                                                }
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                            let _ = writer
                                .create_element("Sections")
                                .write_inner_content(|writer| {
                                    for section in self.schema.sections.iter() {
                                        let _ = writer
                                            .create_element("Section")
                                            .with_attribute(("key", section.key.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                let _ = writer
                                                    .create_element("Title")
                                                    .write_text_content(BytesText::new(section.title.to_string().as_str()));
                                                match &section.title_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Title-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &section.description_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Description-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &section.metadata {
                                                    None => {},
                                                    Some(metadata) => {
                                                        for meta in metadata.iter() {
                                                            let _ = writer
                                                                .create_element("Metadata")
                                                                .with_attribute(("key", meta.key.to_string().as_str()))
                                                                .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                            let _ = writer
                                .create_element("Screens")
                                .write_inner_content(|writer| {
                                    for screen in self.schema.screens.iter() {
                                        let _ = writer
                                            .create_element("Screen")
                                            .with_attribute(("key", screen.key.to_string().as_str()))
                                            .with_attribute(("level", screen.level.to_string().as_str()))
                                            .write_inner_content(|writer| {
                                                let _ = writer
                                                    .create_element("Title")
                                                    .write_text_content(BytesText::new(screen.title.to_string().as_str()));
                                                let _ = writer
                                                    .create_element("Position")
                                                    .write_text_content(BytesText::new(screen.position.to_string().as_str()));
                                                match &screen.title_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Title-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &screen.description_locals {
                                                    None => {},
                                                    Some(locals) => {
                                                        for local in locals.iter() {
                                                            let _ = writer
                                                                .create_element("Description-Local")
                                                                .with_attribute(("lang", local.lang.to_string().as_str()))
                                                                .write_text_content(BytesText::new(local.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &screen.metadata {
                                                    None => {},
                                                    Some(metadata) => {
                                                        for meta in metadata.iter() {
                                                            let _ = writer
                                                                .create_element("Metadata")
                                                                .with_attribute(("key", meta.key.to_string().as_str()))
                                                                .write_text_content(BytesText::new(meta.value.to_string().as_str()))?;
                                                        }
                                                    }
                                                }
                                                match &screen.grid {
                                                    None => {},
                                                    Some(grid) => {
                                                        let _ = writer
                                                        .create_element("Grid")
                                                        .write_inner_content(|writer| {
                                                            let _ = writer
                                                                .create_element("Line-Height")
                                                                .write_text_content(BytesText::new(grid.line_height.to_string().as_str()));
                                                            let _ = writer
                                                                .create_element("Common")
                                                                .write_inner_content(|writer| {
                                                                    for section in grid.common.iter() {
                                                                        let _ = writer
                                                                            .create_element("Section")
                                                                            .with_attribute(("key", section.key.to_string().as_str()))
                                                                            .with_attribute(("position", section.position.to_string().as_str()))
                                                                            .write_inner_content(|writer| {
                                                                                for column in section.columns.iter() {
                                                                                    match column {
                                                                                        CommonColumn::ColumnIdentifier(col) => {
                                                                                            let _ = writer.create_element("Column-Identifier")
                                                                                                .with_attribute(("key", col.key.to_string().as_str()))
                                                                                                .with_attribute(("position", col.position.to_string().as_str()))
                                                                                                .with_attribute(("width", col.width.as_deref().unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("fixed", col.fixed.unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("read-only", col.read_only.unwrap_or_default().to_string().as_str()))
                                                                                                .write_empty();
                                                                                        },
                                                                                        CommonColumn::ColumnClassification(col) => {
                                                                                            let _ = writer
                                                                                                .create_element("Column-Classification")
                                                                                                .with_attribute(("key", col.key.to_string().as_str()))
                                                                                                .with_attribute(("position", col.position.to_string().as_str()))
                                                                                                .with_attribute(("width", col.width.as_deref().unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("fixed", col.fixed.unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("read-only", col.read_only.unwrap_or_default().to_string().as_str()))
                                                                                                .write_empty();
                                                                                        },
                                                                                        CommonColumn::ColumnConditionalFormatting(col) => {
                                                                                            let _ = writer
                                                                                                .create_element("Column-Conditional-Formatting")
                                                                                                .with_attribute(("key", col.key.to_string().as_str()))
                                                                                                .with_attribute(("position", col.position.to_string().as_str()))
                                                                                                .with_attribute(("width", col.width.as_deref().unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("fixed", col.fixed.unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("read-only", col.read_only.unwrap_or_default().to_string().as_str()))
                                                                                                .write_empty();
                                                                                        },
                                                                                        CommonColumn::ColumnField(col) => {
                                                                                            let _ = writer
                                                                                                .create_element("Column-Field")
                                                                                                .with_attribute(("key", col.key.to_string().as_str()))
                                                                                                .with_attribute(("position", col.position.to_string().as_str()))
                                                                                                .with_attribute(("width", col.width.as_deref().unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("fixed", col.fixed.unwrap_or_default().to_string().as_str()))
                                                                                                .with_attribute(("read-only", col.read_only.unwrap_or_default().to_string().as_str()))
                                                                                                .write_empty();
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Ok(())
                                                                            });
                                                                    }
                                                                    Ok(())
                                                                });
                                                            for specific in grid.specifics.iter() {
                                                                let _ = writer
                                                                    .create_element("Specific")
                                                                    .with_attribute(("classification", specific.classification.to_string().as_str()))
                                                                    .with_attribute(("category", specific.category.to_string().as_str()))
                                                                    .write_inner_content(|writer| {
                                                                        for section in specific.sections.iter() {
                                                                            let _ = writer
                                                                                .create_element("Section")
                                                                                .with_attribute(("key", section.key.to_string().as_str()))
                                                                                .with_attribute(("position", section.position.to_string().as_str()))
                                                                                .write_inner_content(|writer| {
                                                                                    for column in section.columns.iter() {
                                                                                        let _ = writer
                                                                                            .create_element("Column-Field")
                                                                                            .with_attribute(("key", column.key.to_string().as_str()))
                                                                                            .with_attribute(("position", column.position.to_string().as_str()))
                                                                                            .with_attribute(("width", column.width.as_deref().unwrap_or_default().to_string().as_str()))
                                                                                            .with_attribute(("fixed", column.fixed.unwrap_or_default().to_string().as_str()))
                                                                                            .with_attribute(("read-only", column.read_only.unwrap_or_default().to_string().as_str()))
                                                                                            .write_empty();
                                                                                    }
                                                                                    Ok(())
                                                                                });
                                                                        }
                                                                        Ok(())
                                                                    });
                                                            }
                                                            Ok(())
                                                        }); 
                                                    }
                                                }
                                                
                                                Ok(())
                                            });
                                    }
                                    Ok(())
                                });
                        Ok(())
                    });
                
                
                Ok(())
            }).unwrap();
    }
}
