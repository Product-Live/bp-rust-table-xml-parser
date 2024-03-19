use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
// use std::error::Error;
use quick_xml::Error;
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};

use crate::table_structs::{AttributeType, CommonColumn, DataType, Table};

pub struct TableXmlWriter {}

impl TableXmlWriter {
    pub fn write(table: &Table, path: &str) -> Result<(), Error> {
        // let mut buffer = File::create(path)?;
        let mut writer = Writer::new(BufWriter::new(File::create(path).unwrap()));

        writer
            .create_element("Table")
            .with_attribute(("key", table.key.to_owned().as_str()))
            .write_inner_content::<_, Error>(|writer| {
                writer
                    .create_element("Title")
                    .write_text_content(BytesText::new(table.title.to_owned().as_str()))?
                    .create_element("Color")
                    .write_text_content(BytesText::new(table.color.to_owned().as_str()))?
                    .create_element("Position")
                    .write_text_content(BytesText::new(table.position.to_string().as_str()))?;
                match &table.description {
                    None => (),
                    Some(description) => {
                        writer
                            .create_element("Description")
                            .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                    }
                }
                match &table.title_locals {
                    None => {},
                    Some(locals) => {
                        for local in locals.iter() {
                            writer
                                .create_element("Title-Local")
                                .with_attribute(("lang", local.lang.to_owned().as_str()))
                                .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                        }
                    }
                }
                match &table.description_locals {
                    None => {},
                    Some(locals) => {
                        for local in locals.iter() {
                            writer
                                .create_element("Description-Local")
                                .with_attribute(("lang", local.lang.to_owned().as_str()))
                                .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                        }
                    }
                }
                writer.create_element("Schema")
                    .write_inner_content::<_,Error>(|writer| {
                        writer
                            .create_element("Levels")
                            .write_inner_content::<_,Error>(|writer| {
                                for level in table.schema.levels.iter() {
                                    let _ = writer
                                        .create_element("Level")
                                        .with_attribute(("key", level.key.to_owned().as_str()))
                                        .with_attribute(("index", level.index.to_string().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(level.title.to_owned().as_str()))?;
                                            match &level.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &level.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &level.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &level.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Partitions")
                            .write_inner_content::<_,Error>(|writer| {
                                for partition in table.schema.partitions.iter() {
                                    writer
                                        .create_element("Partition")
                                        .with_attribute(("key", partition.key.to_owned().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(partition.title.to_owned().as_str()))?
                                                .create_element("Position")
                                                .write_text_content(BytesText::new(partition.position.to_string().as_str()))?;
                                            match &partition.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &partition.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &partition.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &partition.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Identifiers")
                            .write_inner_content::<_,Error>(|writer| {
                                for identifier in table.schema.identifiers.iter() {
                                    writer
                                        .create_element("Identifier")
                                        .with_attribute(("key", identifier.key.to_owned().as_str()))
                                        .with_attribute(("index", identifier.index.to_string().as_str()))
                                        .with_attribute(("level", identifier.level.to_owned().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(identifier.title.to_owned().as_str()))?;
                                            match &identifier.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &identifier.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &identifier.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &identifier.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Classifications")
                            .write_inner_content::<_,Error>(|writer| {
                                for classification in table.schema.classifications.iter() {
                                    writer
                                        .create_element("Classification")
                                        .with_attribute(("key", classification.key.to_owned().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(classification.title.to_owned().as_str()))?
                                                .create_element("Categories")
                                                .write_inner_content::<_,Error>(|writer| {
                                                    for category in classification.categories.iter() {
                                                        let category_el = match &category.parent {
                                                            None => {
                                                                writer
                                                                .create_element("Category")
                                                                .with_attribute(("key", category.key.to_owned().as_str()))
                                                            }
                                                            Some(parent) => {
                                                                writer
                                                                .create_element("Category")
                                                                .with_attribute(("key", category.key.to_owned().as_str()))
                                                                .with_attribute(("parent", parent.to_owned().as_str()))
                                                            }
                                                        };
                                                        category_el.write_inner_content::<_,Error>(|writer| {
                                                                writer
                                                                    .create_element("Title")
                                                                    .write_text_content(BytesText::new(category.title.to_owned().as_str()))?;
                                                                match &category.description {
                                                                    None => (),
                                                                    Some(description) => {
                                                                        writer
                                                                            .create_element("Description")
                                                                            .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                                    }
                                                                }
                                                                match &category.title_locals {
                                                                    None => (),
                                                                    Some(locals) => {
                                                                        for local in locals.iter() {
                                                                            writer
                                                                                .create_element("Title-Local")
                                                                                .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                                .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                        }
                                                                    }
                                                                }
                                                                match &category.description_locals {
                                                                    None => (),
                                                                    Some(locals) => {
                                                                        for local in locals.iter() {
                                                                            writer
                                                                                .create_element("Description-Local")
                                                                                .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                                .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                        }
                                                                    }
                                                                }
                                                                match &category.metadata {
                                                                    None => (),
                                                                    Some(metadata) => {
                                                                        for meta in metadata.iter() {
                                                                            writer
                                                                                .create_element("Metadata")
                                                                                .with_attribute(("key", meta.key.to_owned().as_str()))
                                                                                .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                                        }
                                                                    }
                                                                }
                                                                Ok(())
                                                            })?;
                                                    }
                                                    Ok(())
                                                })?;
                                            match &classification.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &classification.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &classification.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &classification.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Fields")
                            .write_inner_content::<_,Error>(|writer| {
                                for field in table.schema.fields.iter() {
                                    writer
                                        .create_element("Field")
                                        .with_attribute(("key", field.key.to_owned().as_str()))
                                        .with_attribute(("type", field.data_type.to_string().as_str()))
                                        .with_attribute(("level", field.level.to_owned().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(field.title.to_owned().as_str()))?;
                                            match &field.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &field.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &field.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &field.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &field.data_type {
                                                DataType::Number => {
                                                    match &field.prefix {
                                                        None => (),
                                                        Some(prefix) => {
                                                            writer
                                                                .create_element("Prefix")
                                                                .write_text_content(BytesText::new(prefix.to_owned().as_str()))?;
                                                        }
                                                    }
                                                    match &field.suffix {
                                                        None => (),
                                                        Some(suffix) => {
                                                            writer
                                                                .create_element("Suffix")
                                                                .write_text_content(BytesText::new(suffix.to_owned().as_str()))?;
                                                        }
                                                    }
                                                    match &field.precision {
                                                        None => (),
                                                        Some(precision) => {
                                                            writer
                                                                .create_element("Precision")
                                                                .write_text_content(BytesText::new(precision.to_string().as_str()))?;
                                                        }
                                                    }
                                                    match &field.suffixes {
                                                        None => (),
                                                        Some(suffixes) => {
                                                            if suffixes.len() > 0 {
                                                                writer
                                                                    .create_element("Suffixes")
                                                                    .write_inner_content::<_,Error>(|writer| {
                                                                        for suffix in suffixes.iter() {
                                                                            let suffix_el =  match suffix.default {
                                                                                None => {
                                                                                    writer
                                                                                    .create_element("Suffix")
                                                                                    .with_attribute(("key", suffix.key.to_owned().as_str()))
                                                                                },
                                                                                Some(default) => {
                                                                                    writer
                                                                                    .create_element("Suffix")
                                                                                    .with_attribute(("key", suffix.key.to_owned().as_str()))
                                                                                    .with_attribute(("default", default.to_string().as_str()))
                                                                                }
                                                                            };
                                                                            suffix_el.write_inner_content::<_,Error>(|writer| {
                                                                                writer
                                                                                    .create_element("Title")
                                                                                    .write_text_content(BytesText::new(suffix.title.to_owned().as_str()))?;
                                                                                match &suffix.description {
                                                                                    None => (),
                                                                                    Some(description) => {
                                                                                        writer
                                                                                            .create_element("Description")
                                                                                            .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                                                    }
                                                                                }
                                                                                match &suffix.title_locals {
                                                                                    None => (),
                                                                                    Some(locals) => {
                                                                                        for local in locals.iter() {
                                                                                            writer
                                                                                                .create_element("Title-Local")
                                                                                                .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                                                .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match &suffix.description_locals {
                                                                                    None => (),
                                                                                    Some(locals) => {
                                                                                        for local in locals.iter() {
                                                                                            writer
                                                                                                .create_element("Description-Local")
                                                                                                .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                                                .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match &suffix.metadata {
                                                                                    None => (),
                                                                                    Some(metadata) => {
                                                                                        for meta in metadata.iter() {
                                                                                            writer
                                                                                                .create_element("Metadata")
                                                                                                .with_attribute(("key", meta.key.to_owned().as_str()))
                                                                                                .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Ok(())
                                                                            })?;
                                                                        }
                                                                        Ok(())
                                                                    })?;
                                                            }
                                                        }
                                                    }
                                                }
                                                DataType::SingleSelect | DataType::MultipleSelect | DataType::MultipleSelectQuantified | DataType::MultipleSelectQuantifiedWithComments => {
                                                    match &field.options {
                                                        None => (),
                                                        Some(options) => {
                                                            if options.len() > 0 {
                                                                writer.create_element("Options")
                                                                .write_inner_content::<_,Error>(|writer| {
                                                                    for option in options.iter() {
                                                                        let option_el = match &option.color  {
                                                                            None => {
                                                                                writer
                                                                                .create_element("Option")
                                                                                .with_attribute(("key", option.key.to_owned().as_str()))
                                                                            },
                                                                            Some(color) => {
                                                                                writer
                                                                                .create_element("Option")
                                                                                .with_attribute(("key", option.key.to_owned().as_str()))
                                                                                .with_attribute(("color", color.to_owned().as_str()))
                                                                            }
                                                                        };
                                                                        option_el.write_inner_content::<_,Error>(|writer| {
                                                                            writer
                                                                                .create_element("Title")
                                                                                .write_text_content(BytesText::new(option.title.to_owned().as_str()))?;
                                                                            match &option.description {
                                                                                None => (),
                                                                                Some(description) => {
                                                                                    writer
                                                                                        .create_element("Description")
                                                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                                                }
                                                                            }
                                                                            match &option.title_locals {
                                                                                None => (),
                                                                                Some(locals) => {
                                                                                    for local in locals.iter() {
                                                                                        writer
                                                                                            .create_element("Title-Local")
                                                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match &option.description_locals {
                                                                                None => (),
                                                                                Some(locals) => {
                                                                                    for local in locals.iter() {
                                                                                        writer
                                                                                            .create_element("Description-Local")
                                                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match &option.metadata {
                                                                                None => (),
                                                                                Some(metadata) => {
                                                                                    for meta in metadata.iter() {
                                                                                        writer
                                                                                            .create_element("Metadata")
                                                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                                                    }
                                                                                }
                                                                            }
                                                                            Ok(())
                                                                        })?;
                                                                    }
                                                                    Ok(())
                                                                })?;
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => ()
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Matrix")
                            .write_inner_content::<_, Error>(|writer| {
                                writer
                                    .create_element("Common")
                                    .write_inner_content::<_, Error>(|writer| {
                                        for field in table.schema.matrix.common.iter() {
                                            writer
                                                .create_element("Field")
                                                .with_attribute(("key", field.key.to_owned().as_str()))
                                                .write_empty()?;
                                        }
                                        Ok(())
                                    })?;
                                for specific in table.schema.matrix.specifics.iter() {
                                    writer
                                        .create_element("Specific")
                                        .with_attribute(("classification", specific.classification.to_owned().as_str()))
                                        .with_attribute(("category", specific.category.to_owned().as_str()))
                                        .write_inner_content::<_, Error>(|writer| {
                                            for field in specific.fields.iter() {
                                                writer
                                                    .create_element("Field")
                                                    .with_attribute(("key", field.key.to_owned().as_str()))
                                                    .write_empty()?;
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Conditional-Formattings")
                            .write_inner_content::<_, Error>(|writer| {
                                for conditional_formatting in table.schema.conditional_formattings.iter() {
                                    writer
                                        .create_element("Conditional-Formatting")
                                        .with_attribute(("key", conditional_formatting.key.to_owned().as_str()))
                                        .with_attribute(("level", conditional_formatting.level.to_owned().as_str()))
                                        .write_inner_content::<_, Error>(|writer| {
                                            writer
                                        .create_element("Title")
                                        .write_text_content(BytesText::new(&conditional_formatting.title))?;
                                    match &conditional_formatting.description {
                                        None => (),
                                        Some(description) => {
                                            writer
                                                .create_element("Description")
                                                .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                        }
                                    }
                                    match &conditional_formatting.title_locals {
                                        None => (),
                                        Some(locals) => {
                                            for local in locals.iter() {
                                                writer
                                                    .create_element("Title-Local")
                                                    .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                    .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                            }
                                        }
                                    }
                                    match &conditional_formatting.description_locals {
                                        None => (),
                                        Some(locals) => {
                                            for local in locals.iter() {
                                                writer
                                                    .create_element("Description-Local")
                                                    .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                    .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                            }
                                        }
                                    }
                                    match &conditional_formatting.metadata {
                                        None => (),
                                        Some(metadata) => {
                                            for meta in metadata.iter() {
                                                writer
                                                    .create_element("Metadata")
                                                    .with_attribute(("key", meta.key.to_owned().as_str()))
                                                    .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                            }
                                        }
                                    }
                                            writer
                                                .create_element("Default-Status")
                                                .with_attribute(("key", conditional_formatting.default_status.key.to_owned().as_str()))
                                                .write_inner_content::<_, Error>(|writer| {
                                                    writer
                                                        .create_element("Title")
                                                        .write_text_content(BytesText::new(&conditional_formatting.default_status.title.to_owned().as_str()))?;
                                                    writer
                                                        .create_element("Color")
                                                        .write_text_content(BytesText::new(conditional_formatting.default_status.color.to_owned().as_str()))?;
                                                    match &conditional_formatting.default_status.description {
                                                        None => (),
                                                        Some(description) => {
                                                            writer
                                                                .create_element("Description")
                                                                .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                        }
                                                    }
                                                    match &conditional_formatting.default_status.title_locals {
                                                        None => (),
                                                        Some(locals) => {
                                                            for local in locals.iter() {
                                                                writer
                                                                    .create_element("Title-Local")
                                                                    .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                    .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                            }
                                                        }
                                                    }
                                                    match &conditional_formatting.default_status.description_locals {
                                                        None => (),
                                                        Some(locals) => {
                                                            for local in locals.iter() {
                                                                writer
                                                                    .create_element("Description-Local")
                                                                    .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                    .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                            }
                                                        }
                                                    }
                                                    match &conditional_formatting.default_status.metadata {
                                                        None => (),
                                                        Some(metadata) => {
                                                            for meta in metadata.iter() {
                                                                writer
                                                                    .create_element("Metadata")
                                                                    .with_attribute(("key", meta.key.to_owned().as_str()))
                                                                    .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                            }
                                                        }
                                                    }
                                                    Ok(())
                                                })?;
                                            writer
                                                .create_element("Statuses")
                                                .write_inner_content::<_, Error>(|writer| {
                                                    for status in conditional_formatting.statuses.iter() {
                                                        writer
                                                            .create_element("Status")
                                                            .with_attribute(("key", status.key.to_owned().as_str()))
                                                            .write_inner_content::<_, Error>(|writer| {
                                                                writer
                                                                .create_element("Title")
                                                                .write_text_content(BytesText::new(status.title.to_owned().as_str()))?;
                                                            writer
                                                                .create_element("Color")
                                                                .write_text_content(BytesText::new(status.color.to_owned().as_str()))?;
                                                            writer
                                                                .create_element("Priority")
                                                                .write_text_content(BytesText::new(status.priority.to_string().as_str()))?;
                                                            match &status.description {
                                                                None => (),
                                                                Some(description) => {
                                                                    writer
                                                                        .create_element("Description")
                                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                                }
                                                            }
                                                            match &status.title_locals {
                                                                None => (),
                                                                Some(locals) => {
                                                                    for local in locals.iter() {
                                                                        writer
                                                                            .create_element("Title-Local")
                                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                    }
                                                                }
                                                            }
                                                            match &status.description_locals {
                                                                None => (),
                                                                Some(locals) => {
                                                                    for local in locals.iter() {
                                                                        writer
                                                                            .create_element("Description-Local")
                                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                                    }
                                                                }
                                                            }
                                                            match &status.metadata {
                                                                None => (),
                                                                Some(metadata) => {
                                                                    for meta in metadata.iter() {
                                                                        writer
                                                                            .create_element("Metadata")
                                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                                    }
                                                                }
                                                            }
                                                            writer
                                                                .create_element("Rules")
                                                                .write_inner_content::<_, Error>(|writer| {
                                                                    if status.rules.common.len() > 0 {
                                                                        writer
                                                                            .create_element("Common")
                                                                            .write_inner_content::<_, Error>(|writer| {
                                                                                for attribute in status.rules.common.iter() {
                                                                                    match attribute.attribute_type {
                                                                                        AttributeType::Identifier => {
                                                                                            writer
                                                                                                .create_element("Identifier")
                                                                                                .with_attribute(("key", attribute.key.to_owned().as_str()))
                                                                                                .write_empty()?;
                                                                                        },
                                                                                        AttributeType::Classification => {
                                                                                            writer
                                                                                                .create_element("Classification")
                                                                                                .with_attribute(("key", attribute.key.to_owned().as_str()))
                                                                                                .write_empty()?;
                                                                                        },
                                                                                        AttributeType::Field => {
                                                                                            writer
                                                                                                .create_element("Field")
                                                                                                .with_attribute(("key", attribute.key.to_owned().as_str()))
                                                                                                .write_empty()?;
                                                                                        },
                                                                                    }
                                                                                }
                                                                                Ok(())
                                                                            })?;
                                                                    }
                                                                    for specific in status.rules.specifics.iter() {
                                                                        writer
                                                                            .create_element("Specific")
                                                                            .with_attribute(("classification", specific.classification.to_owned().as_str()))
                                                                            .with_attribute(("category", specific.category.to_owned().as_str()))
                                                                            .write_inner_content::<_, Error>(|writer| {
                                                                                for attribute in specific.attributes.iter() {
                                                                                    writer
                                                                                        .create_element("Field")
                                                                                        .with_attribute(("key", attribute.key.to_owned().as_str()))
                                                                                        .write_empty()?;
                                                                                }
                                                                                Ok(())
                                                                            })?;
                                                                    }
                                                                    Ok(())
                                                                })?;
                                                                Ok(())
                                                            })?;
                                                    }
                                                    Ok(())
                                                })?;
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Sections")
                            .write_inner_content::<_,Error>(|writer| {
                                for section in table.schema.sections.iter() {
                                    writer
                                        .create_element("Section")
                                        .with_attribute(("key", section.key.to_owned().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(section.title.to_owned().as_str()))?;
                                            match &section.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &section.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &section.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &section.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?
                            .create_element("Screens")
                            .write_inner_content::<_,Error>(|writer| {
                                for screen in table.schema.screens.iter() {
                                    writer
                                        .create_element("Screen")
                                        .with_attribute(("key", screen.key.to_owned().as_str()))
                                        .write_inner_content::<_,Error>(|writer| {
                                            writer
                                                .create_element("Title")
                                                .write_text_content(BytesText::new(screen.title.to_owned().as_str()))?
                                                .create_element("Position")
                                                .write_text_content(BytesText::new(screen.position.to_string().as_str()))?;
                                            match &screen.description {
                                                None => (),
                                                Some(description) => {
                                                    writer
                                                        .create_element("Description")
                                                        .write_text_content(BytesText::new(description.to_owned().as_str()))?;
                                                }
                                            }
                                            match &screen.title_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Title-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &screen.description_locals {
                                                None => (),
                                                Some(locals) => {
                                                    for local in locals.iter() {
                                                        writer
                                                            .create_element("Description-Local")
                                                            .with_attribute(("lang", local.lang.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(local.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            match &screen.metadata {
                                                None => (),
                                                Some(metadata) => {
                                                    for meta in metadata.iter() {
                                                        writer
                                                            .create_element("Metadata")
                                                            .with_attribute(("key", meta.key.to_owned().as_str()))
                                                            .write_text_content(BytesText::new(meta.value.to_owned().as_str()))?;
                                                    }
                                                }
                                            }
                                            writer
                                                .create_element("Grid")
                                                .write_inner_content::<_, Error>(|writer| {
                                                    writer
                                                        .create_element("Line-Height")
                                                        .write_text_content(BytesText::new(screen.grid.line_height.to_owned().as_str()))?
                                                        .create_element("Common")
                                                        .write_inner_content::<_,Error>(|writer| {
                                                            for section in screen.grid.common.iter() {
                                                                writer
                                                                    .create_element("Section")
                                                                    .with_attribute(("key", section.key.to_owned().as_str()))
                                                                    .with_attribute(("position", section.position.to_string().as_str()))
                                                                    .write_inner_content::<_, Error>(|writer| {
                                                                        for column in section.columns.iter() {
                                                                            match column {
                                                                                CommonColumn::ColumnIdentifier(column) => {
                                                                                    let mut column_el = writer.create_element("Column-Identifier");
                                                                                    column_el = column_el.with_attribute(("key", column.key.to_owned().as_str()));
                                                                                    column_el = column_el.with_attribute(("position", column.position.to_string().as_str()));
                                                                                    match &column.fixed {
                                                                                        Some(fixed) => if fixed.to_owned() {
                                                                                            column_el = column_el.with_attribute(("fixed", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.width {
                                                                                        Some(width) => if width != "" && width != "MEDIUM" {
                                                                                            column_el = column_el.with_attribute(("width", width.to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.read_only {
                                                                                        Some(read_only) => if read_only.to_owned() {
                                                                                            column_el = column_el.with_attribute(("read-only", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    column_el.write_empty()?;
                                                                                },
                                                                                CommonColumn::ColumnClassification(column) =>  {
                                                                                    let mut column_el = writer.create_element("Column-Classification");
                                                                                    column_el = column_el.with_attribute(("key", column.key.to_owned().as_str()));
                                                                                    column_el = column_el.with_attribute(("position", column.position.to_string().as_str()));
                                                                                    match &column.fixed {
                                                                                        Some(fixed) => if fixed.to_owned() {
                                                                                            column_el = column_el.with_attribute(("fixed", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.width {
                                                                                        Some(width) => if width != "" && width != "MEDIUM" {
                                                                                            column_el = column_el.with_attribute(("width", width.to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.read_only {
                                                                                        Some(read_only) => if read_only.to_owned() {
                                                                                            column_el = column_el.with_attribute(("read-only", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    column_el.write_empty()?;
                                                                                },
                                                                                CommonColumn::ColumnConditionalFormatting(column) => {
                                                                                    let mut column_el = writer.create_element("Column-Conditional-Formatting");
                                                                                    column_el = column_el.with_attribute(("key", column.key.to_owned().as_str()));
                                                                                    column_el = column_el.with_attribute(("position", column.position.to_string().as_str()));
                                                                                    match &column.fixed {
                                                                                        Some(fixed) => if fixed.to_owned() {
                                                                                            column_el = column_el.with_attribute(("fixed", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.width {
                                                                                        Some(width) => if width != "" && width != "MEDIUM" {
                                                                                            column_el = column_el.with_attribute(("width", width.to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.read_only {
                                                                                        Some(read_only) => if read_only.to_owned() {
                                                                                            column_el = column_el.with_attribute(("read-only", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    column_el.write_empty()?;
                                                                                },
                                                                                CommonColumn::ColumnField(column) => {
                                                                                    let mut column_el = writer.create_element("Column-Field");
                                                                                    column_el = column_el.with_attribute(("key", column.key.to_owned().as_str()));
                                                                                    column_el = column_el.with_attribute(("position", column.position.to_string().as_str()));
                                                                                    match &column.fixed {
                                                                                        Some(fixed) => if fixed.to_owned() {
                                                                                            column_el = column_el.with_attribute(("fixed", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.width {
                                                                                        Some(width) => if width != "" && width != "MEDIUM" {
                                                                                            column_el = column_el.with_attribute(("width", width.to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.read_only {
                                                                                        Some(read_only) => if read_only.to_owned() {
                                                                                            column_el = column_el.with_attribute(("read-only", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    column_el.write_empty()?;
                                                                                },
                                                                            }
                                                                        }
                                                                        Ok(())
                                                                    })?;
                                                            }
                                                            Ok(())
                                                        })?;
                                                        for specific in screen.grid.specifics.iter() {
                                                            writer
                                                                .create_element("Specific")
                                                                .with_attribute(("classification", specific.classification.to_owned().as_str()))
                                                                .with_attribute(("category", specific.category.to_owned().as_str()))
                                                                .write_inner_content::<_, Error>(|writer| {
                                                                    for section in specific.sections.iter() {
                                                                        writer
                                                                            .create_element("Section")
                                                                            .with_attribute(("key", section.key.to_owned().as_str()))
                                                                            .with_attribute(("position", section.position.to_string().as_str()))
                                                                            .write_inner_content::<_, Error>(|writer| {
                                                                                for column in section.columns.iter() {
                                                                                    let mut column_el = writer
                                                                                        .create_element("Column-Field")
                                                                                        .with_attribute(("key", column.key.to_owned().as_str()))
                                                                                        .with_attribute(("position", column.key.to_owned().as_str()));
                                                                                    match &column.fixed {
                                                                                        Some(fixed) => {
                                                                                            if fixed.to_owned() == true {
                                                                                                column_el = column_el.with_attribute(("fixed", "true".to_owned().as_str()));
                                                                                            }
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.width {
                                                                                        Some(width) => if width != "" && width != "MEDIUM" {
                                                                                            column_el = column_el.with_attribute(("width", width.to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    match &column.read_only {
                                                                                        Some(read_only) => if read_only.to_owned() {
                                                                                            column_el = column_el.with_attribute(("read-only", "true".to_owned().as_str()));
                                                                                        },
                                                                                        None => (),
                                                                                    }
                                                                                    column_el.write_empty()?;
                                                                                }
                                                                                Ok(())
                                                                            })?;
                                                                    }
                                                                    Ok(())
                                                                })?;
                                                        }
                                                    Ok(())
                                                })?;
                                            Ok(())
                                        })?;
                                }
                                Ok(())
                            })?;
                        Ok(())
                    })?;
                Ok(())
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{table_xml_parser::TableXmlParser, table_xml_writer::TableXmlWriter};

    #[test]
    fn write_file() {
        let table_xml_parser = TableXmlParser::read("./tests/input.xml").unwrap();
        match TableXmlWriter::write(&table_xml_parser.table, "./tests/output.xml") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
