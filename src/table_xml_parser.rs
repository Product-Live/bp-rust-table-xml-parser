use std::{collections::HashMap, error::Error, fs::File, io::BufReader, str::from_utf8};

use quick_xml::{events::Event, Reader};
use serde::{Deserialize, Serialize};

use crate::{
    table_structs::{
        Action, AttributeType, Category, Classification, Column, CommonAttributeRules,
        CommonColumn, CommonSection, Condition, ConditionGroup, ConditionalFormatting, Control,
        DataType, DefaultStatus, Field, Formula, GridSpecific, Identifier, Level, Local,
        MatrixField, MatrixSpecific, Metadata, OptionRule, Partition, Rule, Screen, Section,
        SelectOption, SpecificAttributeRules, SpecificRules, SpecificSection, Status, Suffix,
        Table, UseSuffix,
    },
    utils::get_attributes,
};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub code: String,
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableXmlParser {
    pub table: Table,
    pub warnings: Vec<Log>,
    pub errors: Vec<Log>,
}

impl TableXmlParser {
    pub fn read(path: &str) -> Result<TableXmlParser, Box<dyn Error>> {
        let mut xml_parser = TableXmlParser {
            table: Table::new(),
            warnings: vec![],
            errors: vec![],
        };
        xml_parser.process_xml(path)?;
        Ok(xml_parser)
    }
    // Parse xml file
    fn process_xml(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut reader: Reader<BufReader<File>> = Reader::from_file(path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut table_element_found = false;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Eof => break,

                Event::Start(ev) => match ev.name().as_ref() {
                    b"Table" => {
                        table_element_found = true;
                        self.process_table(get_attributes(ev.attributes())?, &mut reader, &mut buf)?
                    }
                    name => self.errors.push(Log {
                        code: "WRONG_FIRST_ELEMENT".to_owned(),
                        message:
                        format!(
                            "File must start with the Table element. The element name found is '{}'.",
                            from_utf8(name)?),
                    }),
                },

                Event::Empty(ev) => match ev.name().as_ref() {
                    b"Table" => {
                        self.errors.push(Log {
                            code: "EMPTY_TABLE_ELEMENT".to_owned(),
                            message: "The table element is empty".to_owned(),
                        });
                    },
                    name => self.errors.push(Log {
                        code: "WRONG_FIRST_ELEMENT".to_owned(),
                        message:
                        format!(
                            "File must start with the Table element. The element name found is '{}'.",
                            from_utf8(name)?),
                    }),
                }

                _ => (),
            }
            buf.clear();
        }

        if table_element_found == false {
            self.errors.push(Log {
                code: "MISSING_ELEMENT_TABLE".to_owned(),
                message:
                    "The xml file must start with the Table. It's seams that the file is empty"
                        .to_owned(),
            });
        }

        Ok(())
    }

    fn process_table(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        self.table.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => self.table.title = self.handle_text(reader, buf)?,
                        b"Position" => self.table.position = self.handle_number(reader, buf)?,
                        b"Color" => self.table.color = self.handle_text(reader, buf)?,
                        b"Description" => {
                            self.table.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => {
                                    match self.handle_optional_local(lang, reader, buf)? {
                                        Some(local) => self.table.add_title_local(Some(local)),
                                        None => (),
                                    }
                                }
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => {
                                    match self.handle_optional_local(lang, reader, buf)? {
                                        Some(local) => {
                                            self.table.add_description_local(Some(local))
                                        }
                                        None => (),
                                    }
                                }
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Schema" => self.process_schema(reader, buf)?,
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Table" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_schema(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Partitions" => self.process_partitions(reader, buf)?,
                    b"Levels" => self.process_levels(reader, buf)?,
                    b"Identifiers" => self.process_identifiers(reader, buf)?,
                    b"Classifications" => self.process_classifications(reader, buf)?,
                    b"Fields" => self.process_fields(reader, buf)?,
                    b"Formulas" => self.process_formulas(reader, buf)?,
                    b"Matrix" => self.process_matrix(reader, buf)?,
                    b"Conditional-Formattings" => {
                        self.process_conditional_formattings(reader, buf)?
                    }
                    b"Sections" => self.process_sections(reader, buf)?,
                    b"Screens" => self.process_screens(reader, buf)?,
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Schema" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }

    fn process_partitions(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Partition" => {
                        self.process_partition(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Partitions" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_partition(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut partition = Partition::new();
        partition.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => partition.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            partition.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => partition.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => partition.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => partition
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Partition" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.partitions.push(partition);
        Ok(())
    }

    fn process_levels(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Level" => {
                        self.process_level(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Levels" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_level(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut level = Level::new();
        level.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        level.index = attributes
            .get("index")
            .unwrap_or(&"0".to_owned())
            .parse()
            .unwrap_or(0);
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => level.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            level.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => level.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => level.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => level
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Level" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.levels.push(level);
        Ok(())
    }

    fn process_identifiers(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Identifier" => {
                        self.process_identifier(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Identifiers" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_identifier(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut identifier = Identifier::new();
        identifier.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        identifier.level = attributes
            .get("level")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        identifier.index = attributes
            .get("index")
            .unwrap_or(&"0".to_owned())
            .parse()
            .unwrap_or(0);
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => identifier.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            identifier.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => identifier.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => identifier.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => identifier
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Identifier" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.identifiers.push(identifier);
        Ok(())
    }

    fn process_classifications(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Classification" => {
                        self.process_classification(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Classifications" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_classification(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut classification = Classification::new();
        classification.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => classification.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            classification.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => classification.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => classification.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => classification
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Categories" => {
                            classification.categories = self.process_categories(reader, buf)?
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Classification" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.classifications.push(classification);
        Ok(())
    }
    fn process_categories(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<Category>, Box<dyn Error>> {
        let mut categories: Vec<Category> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Category" => categories.push(self.process_category(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?),
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Categories" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(categories)
    }
    fn process_category(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Category, Box<dyn Error>> {
        let mut category = Category::new();
        category.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        match attributes.get("parent") {
            Some(parent) => category.parent = Some(parent.to_owned()),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => category.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            category.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => category.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => category.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => category
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Category" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(category)
    }

    fn process_fields(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Field" => {
                        self.process_field(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Fields" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_field(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        let level = attributes
            .get("level")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        let data_type = attributes.get("type");
        if data_type.is_none() {
            Err("Data type is empty, field is skipped")?;
        }
        let mut field = match data_type.unwrap().as_str() {
            "SINGLE-LINE-TEXT" => Field::new(key, level, DataType::SingleLineText),
            "LONG-TEXT" => Field::new(key, level, DataType::LongText),
            "HTML-TEXT" => Field::new(key, level, DataType::HtmlText),
            "NUMBER" => Field::new(key, level, DataType::Number),
            "SINGLE-SELECT" => Field::new(key, level, DataType::SingleSelect),
            "MULTIPLE-SELECT" => Field::new(key, level, DataType::MultipleSelect),
            "MULTIPLE-SELECT-QUANTIFIED" => {
                Field::new(key, level, DataType::MultipleSelectQuantified)
            }
            "MULTIPLE-SELECT-QUANTIFIED-WITH-COMMENTS" => {
                Field::new(key, level, DataType::MultipleSelectQuantifiedWithComments)
            }
            "DATE" => Field::new(key, level, DataType::Date),
            "DATE-TIME" => Field::new(key, level, DataType::DateTime),
            "IMAGE" => Field::new(key, level, DataType::Image),
            "ATTACHMENT" => Field::new(key, level, DataType::Attachment),
            _ => Err("Unvalid data type, field is skipped")?,
        };
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => field.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            field.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => field.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => field.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => field
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Prefix" => field.prefix = self.handle_optional_text(reader, buf)?,
                        b"Suffix" => field.suffix = self.handle_optional_text(reader, buf)?,
                        b"Precision" => {
                            field.precision = self.handle_optional_number(reader, buf)?
                        }
                        b"Options" => self.process_options(&mut field, reader, buf)?,
                        b"Suffixes" => self.process_suffixes(&mut field, reader, buf)?,
                        name => self.add_warning(Log {
                            code: "UNKNOWN_ELEMENT_NAME".to_owned(),
                            message: format!(
                                "Unknown element name '{}' in Field",
                                from_utf8(name)?
                            ),
                        }),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Field" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.fields.push(field);

        Ok(())
    }
    fn process_options(
        &mut self,
        field: &mut Field,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut options: Vec<SelectOption> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Option" => options.push(self.process_option(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?),
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Options" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        field.options = Some(options);
        Ok(())
    }
    fn process_option(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<SelectOption, Box<dyn Error>> {
        let mut option = SelectOption::new();
        option.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        match attributes.get("color") {
            Some(color) => option.color = Some(color.to_owned()),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => option.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            option.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => option.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => option.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => option
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        // name => Err(format!(
                        //     "Unknown element name '{}' within a Option element",
                        //     from_utf8(name).unwrap()
                        // ))?,
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Option" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(option)
    }
    fn process_suffixes(
        &mut self,
        field: &mut Field,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut suffixes: Vec<Suffix> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Suffix" => {
                        suffixes.push(self.process_suffix(
                            get_attributes(ev.attributes())?,
                            reader,
                            buf,
                        )?);
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Suffixes" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        field.suffixes = Some(suffixes);
        Ok(())
    }
    fn process_suffix(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Suffix, Box<dyn Error>> {
        let mut suffix = Suffix::new();
        suffix.key = attributes
            .get("key")
            .unwrap_or(&"UNKNOWN".to_owned())
            .to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => suffix.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            suffix.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => suffix.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => suffix.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => suffix
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Suffix" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(suffix)
    }

    fn process_matrix(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Common" => self.process_matrix_common(reader, buf)?,
                    b"Specific" => {
                        let mut classification: String = "UNKNOWN".to_owned();
                        let mut category: String = "UNKNOWN".to_owned();
                        let attributes = get_attributes(ev.attributes())?;
                        match attributes.get("classification") {
                            Some(value) => classification = value.to_owned(),
                            None => (),
                        }
                        match attributes.get("category") {
                            Some(value) => category = value.to_owned(),
                            None => (),
                        }
                        let mut specific = MatrixSpecific {
                            classification: classification,
                            category: category,
                            fields: vec![],
                        };
                        self.process_matrix_specific(&mut specific, reader, buf)?;
                        self.table.schema.matrix.specifics.push(specific)
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Matrix" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_matrix_common(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) | Event::Empty(ev) => match ev.name().as_ref() {
                    b"Field" => {
                        let mut key: String = "UNKNOWN".to_owned();
                        match get_attributes(ev.attributes())?.get("key") {
                            Some(value) => key = value.to_owned(),
                            None => (),
                        }
                        self.table
                            .schema
                            .matrix
                            .common
                            .push(MatrixField { key: key })
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Common" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_matrix_specific(
        &self,
        specific: &mut MatrixSpecific,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) | Event::Empty(ev) => match ev.name().as_ref() {
                    b"Field" => {
                        let mut key: String = "UNKNOWN".to_owned();
                        match get_attributes(ev.attributes())?.get("key") {
                            Some(value) => key = value.to_owned(),
                            None => (),
                        }
                        specific.fields.push(MatrixField { key: key })
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Specific" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }

    fn process_formulas(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Identifier" => self.process_formula(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                        AttributeType::Identifier,
                    )?,
                    b"Classification" => self.process_formula(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                        AttributeType::Classification,
                    )?,
                    b"Field" => self.process_formula(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                        AttributeType::Field,
                    )?,
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Formulas" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_formula(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
        attribute_type: AttributeType,
    ) -> Result<(), Box<dyn Error>> {
        let mut key = "UNKNOWN".to_owned();
        match attributes.get("key") {
            Some(key_str) => key = key_str.to_owned(),
            None => (),
        }
        let mut formula = Formula {
            attribute_type: attribute_type,
            key: key,
            rules: vec![],
        };
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Rule" => {
                        match self.process_rule(get_attributes(ev.attributes())?, reader, buf)? {
                            Some(rule) => formula.rules.push(rule),
                            None => (),
                        }
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Identifier" | b"Classification" | b"Field" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        if formula.rules.len() > 0 {
            self.table.schema.formulas.push(formula);
        }
        Ok(())
    }
    fn process_rule(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Rule>, Box<dyn Error>> {
        let mut priority: usize = 0;
        match attributes.get("priority") {
            Some(priority_attr) => match priority_attr.parse::<usize>() {
                Ok(p) => priority = p,
                Err(_) => todo!(),
            },
            None => (),
        }
        let mut conditions: Vec<ConditionGroup> = vec![];
        let mut action: Option<Action> = None;
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Conditions" => {
                        conditions = self.process_rule_condition_conditions(reader, buf)?
                    }
                    b"Action" => {
                        action =
                            self.process_action(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Rule" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        if conditions.len() == 0 {
            self.add_warning(Log {
                code: "".to_owned(),
                message: "".to_owned(),
            });
            return Ok(None);
        }
        match action {
            Some(action) => Ok(Some(Rule {
                priority: priority,
                conditions: conditions,
                action: action,
            })),
            None => Ok(None),
        }
    }
    fn process_action(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Action>, Box<dyn Error>> {
        match attributes.get("type") {
            Some(action_type) => match action_type.as_bytes() {
                b"SET_TEXT" => Ok(Some(self.process_action_set_text(reader, buf)?)),
                b"SET_NUMBER" => Ok(Some(self.process_action_set_number(reader, buf)?)),
                b"SET_SELECTABLE_OPTIONS" => Ok(Some(
                    self.process_action_set_selectable_options(reader, buf)?,
                )),
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }
    fn process_action_set_text(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Action, Box<dyn Error>> {
        let mut trim_spaces = false;
        let mut value = "".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Template" => {
                        match get_attributes(ev.attributes())?.get("trim-spaces") {
                            Some(trim_spaces_str) => match trim_spaces_str.as_bytes() {
                                b"true" => trim_spaces = true,
                                _ => (),
                            },
                            None => (),
                        }
                        value = self.process_action_template(reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Action" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(Action::SetTextTemplate {
            trim_spaces: trim_spaces,
            value: value,
        })
    }
    fn process_action_template(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let mut value = "".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::CData(ev) => value = ev.escape()?.unescape()?.into_owned(),
                Event::Text(ev) => value = ev.unescape()?.into_owned(),
                Event::End(ev) => match ev.name().as_ref() {
                    b"Template" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(value)
    }
    fn process_action_set_number(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Action, Box<dyn Error>> {
        let mut precision: usize = 0;
        let mut round: String = "CEILING".to_owned();
        let mut value = "".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Template" => {
                        match get_attributes(ev.attributes())?.get("precision") {
                            Some(precision_str) => match precision_str.parse() {
                                Ok(precision_number) => precision = precision_number,
                                Err(_) => (),
                            },
                            None => (),
                        }
                        match get_attributes(ev.attributes())?.get("round") {
                            Some(round_str) => round = round_str.to_owned(),
                            None => (),
                        }
                        value = self.process_action_template(reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Action" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(Action::SetNumberTemplate {
            precision: precision,
            round: round,
            value: value,
        })
    }
    fn process_action_set_selectable_options(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Action, Box<dyn Error>> {
        let mut values: Vec<String> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Value" => {
                        match self.process_action_set_selectable_options_value(reader, buf)? {
                            Some(value) => values.push(value),
                            None => (),
                        }
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Action" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(Action::SetSelectableOptions { values: values })
    }
    fn process_action_set_selectable_options_value(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let mut value: Option<String> = None;
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => value = Some(ev.unescape()?.into_owned()),
                Event::End(ev) => match ev.name().as_ref() {
                    b"Value" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(value)
    }

    fn process_conditional_formattings(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Conditional-Formatting" => self.process_conditional_formatting(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Conditional-Formattings" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_conditional_formatting(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut conditional_formatting = ConditionalFormatting::new();
        match attributes.get("key") {
            Some(key) => conditional_formatting.key = key.to_owned(),
            None => (),
        }
        match attributes.get("level") {
            Some(level) => conditional_formatting.level = level.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => conditional_formatting.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            conditional_formatting.description =
                                self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => conditional_formatting.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => conditional_formatting.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => conditional_formatting
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Default-Status" => self.process_conditional_formatting_default_status(
                            &mut conditional_formatting,
                            get_attributes(ev.attributes())?,
                            reader,
                            buf,
                        )?,
                        b"Statuses" => self.process_conditional_formatting_statuses(
                            &mut conditional_formatting,
                            reader,
                            buf,
                        )?,
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Conditional-Formatting" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table
            .schema
            .conditional_formattings
            .push(conditional_formatting);

        Ok(())
    }
    fn process_conditional_formatting_default_status(
        &mut self,
        conditional_formatting: &mut ConditionalFormatting,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut default_status = DefaultStatus::new();
        match attributes.get("key") {
            Some(key) => default_status.key = key.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Title" => default_status.title = self.handle_text(reader, buf)?,
                    b"Color" => default_status.color = self.handle_text(reader, buf)?,
                    b"Description" => {
                        default_status.description = self.handle_optional_text(reader, buf)?
                    }
                    b"Title-Local" => {
                        match get_attributes(ev.attributes())?.get("lang") {
                            Some(lang) => default_status
                                .add_title_local(self.handle_optional_local(lang, reader, buf)?),
                            None => (), // Ignore if there is no lang attribute
                        }
                    }
                    b"Description-Local" => {
                        match get_attributes(ev.attributes())?.get("lang") {
                            Some(lang) => default_status.add_description_local(
                                self.handle_optional_local(lang, reader, buf)?,
                            ),
                            None => (), // Ignore if there is no lang attribute
                        }
                    }
                    b"Metadata" => {
                        match get_attributes(ev.attributes())?.get("key") {
                            Some(key) => default_status
                                .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                            None => (), // Ignore if there is no lang attribute
                        }
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Default-Status" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        conditional_formatting.default_status = default_status;
        Ok(())
    }
    fn process_conditional_formatting_statuses(
        &mut self,
        conditional_formatting: &mut ConditionalFormatting,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut statuses: Vec<Status> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Status" => statuses.push(self.process_conditional_formatting_status(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?),
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Statuses" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        conditional_formatting.statuses = statuses;
        Ok(())
    }
    fn process_conditional_formatting_status(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Status, Box<dyn Error>> {
        let mut status = Status::new();
        match attributes.get("key") {
            Some(key) => status.key = key.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => status.title = self.handle_text(reader, buf)?,
                        b"Color" => status.color = self.handle_text(reader, buf)?,
                        b"Priority" => status.priority = self.handle_number(reader, buf)?,
                        b"Description" => {
                            status.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => status.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => status.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => status
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Rules" => self.process_status_rules(&mut status, reader, buf)?,
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Status" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(status)
    }
    fn process_status_rules(
        &mut self,
        status: &mut Status,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Common" => {
                        status.rules.common = self.process_status_rules_common(reader, buf)?
                    }
                    b"Specfic" => match self.process_status_rules_specific(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )? {
                        Some(specific_rules) => status.rules.specifics.push(specific_rules),
                        None => (),
                    },
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Rules" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_status_rules_common(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<CommonAttributeRules>, Box<dyn Error>> {
        let mut common: Vec<CommonAttributeRules> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Identifier" | b"Classification" | b"Field" => {
                        let mut key = "UNKNOWN".to_owned();
                        match get_attributes(ev.attributes())?.get("key") {
                            Some(key_s) => key = key_s.to_owned(),
                            None => (),
                        }
                        let controls = self.process_controls(reader, buf)?;
                        if controls.len() > 0 {
                            common.push(CommonAttributeRules {
                                attribute_type: AttributeType::Identifier,
                                key: key,
                                controls: controls,
                            });
                        }
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Common" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(common)
    }
    fn process_status_rules_specific(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<SpecificRules>, Box<dyn Error>> {
        match attributes.get("classification") {
            Some(classification) => match attributes.get("category") {
                Some(category) => {
                    let mut attributes: Vec<SpecificAttributeRules> = vec![];
                    loop {
                        match reader.read_event_into(buf)? {
                            Event::Start(ev) => match ev.name().as_ref() {
                                b"Field" => {
                                    let mut key = "UNKNOWN".to_owned();
                                    match get_attributes(ev.attributes())?.get("key") {
                                        Some(key_s) => key = key_s.to_owned(),
                                        None => (),
                                    }
                                    let controls = self.process_controls(reader, buf)?;
                                    if controls.len() > 0 {
                                        attributes.push(SpecificAttributeRules {
                                            key: key,
                                            controls: controls,
                                        })
                                    }
                                }
                                _ => (),
                            },
                            Event::End(ev) => match ev.name().as_ref() {
                                b"Specific" => break,
                                _ => (),
                            },
                            _ => (),
                        }
                        buf.clear();
                    }
                    if attributes.len() > 0 {
                        Ok(Some(SpecificRules::new(
                            classification.to_owned(),
                            category.to_owned(),
                            attributes,
                        )))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            },
            None => Ok(None),
        }
    }
    fn process_controls(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<Control>, Box<dyn Error>> {
        let mut controls: Vec<Control> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) | Event::Empty(ev) => match ev.name().as_ref() {
                    b"Rule-Required" => controls.push(Control::RuleRequired),
                    b"Rule-Barcode" => match get_attributes(ev.attributes())?.get("type") {
                        Some(barcode_type) => controls.push(Control::RuleBarcode {
                            barcode_type: barcode_type.to_owned(),
                        }),
                        None => (),
                    },
                    b"Rule-Regex" => match get_attributes(ev.attributes())?.get("regex") {
                        Some(regex) => controls.push(Control::RuleRegex {
                            regex: regex.to_owned(),
                        }),
                        None => (),
                    },
                    b"Rule-Min-Length" => match get_attributes(ev.attributes())?.get("min") {
                        Some(min) => match min.parse() {
                            Ok(min) => controls.push(Control::RuleMinLength { min: min }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Max-Length" => match get_attributes(ev.attributes())?.get("max") {
                        Some(max) => match max.parse() {
                            Ok(max) => controls.push(Control::RuleMaxLength { max: max }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Is-Leaf" => controls.push(Control::RuleIsLeaf),
                    b"Rule-Less-Than" => match get_attributes(ev.attributes())?.get("value") {
                        Some(value) => match value.parse() {
                            Ok(value) => controls.push(Control::RuleLessThan { value: value }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Greater-Than" => match get_attributes(ev.attributes())?.get("value") {
                        Some(value) => match value.parse() {
                            Ok(value) => controls.push(Control::RuleGreaterThan { value: value }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Less-Than-Or-Equal" => {
                        match get_attributes(ev.attributes())?.get("value") {
                            Some(value) => match value.parse() {
                                Ok(value) => {
                                    controls.push(Control::RuleLessThanOrEqual { value: value })
                                }
                                Err(_) => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Greater-Than-Or-Equal" => {
                        match get_attributes(ev.attributes())?.get("value") {
                            Some(value) => match value.parse() {
                                Ok(value) => {
                                    controls.push(Control::RuleGreaterThanOrEqual { value: value })
                                }
                                Err(_) => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Decimal-Places" => {
                        match get_attributes(ev.attributes())?.get("precision") {
                            Some(precision) => match precision.parse() {
                                Ok(precision) => controls.push(Control::RuleDecimalPlaces {
                                    precision: precision,
                                }),
                                Err(_) => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Min-Width-Px" => match get_attributes(ev.attributes())?.get("min") {
                        Some(min) => match min.parse() {
                            Ok(min) => controls.push(Control::RuleMinWidthPx { min: min }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Max-Width-Px" => match get_attributes(ev.attributes())?.get("max") {
                        Some(max) => match max.parse() {
                            Ok(max) => controls.push(Control::RuleMaxWidthPx { max: max }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Min-Height-Px" => match get_attributes(ev.attributes())?.get("min") {
                        Some(min) => match min.parse() {
                            Ok(min) => controls.push(Control::RuleMinHeightPx { min: min }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Max-Height-Px" => match get_attributes(ev.attributes())?.get("max") {
                        Some(max) => match max.parse() {
                            Ok(max) => controls.push(Control::RuleMaxHeightPx { max: max }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Max-Size-Kb" => match get_attributes(ev.attributes())?.get("max") {
                        Some(max) => match max.parse() {
                            Ok(max) => controls.push(Control::RuleMaxSizeKb { max: max }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Extension" => match get_attributes(ev.attributes())?.get("extension") {
                        Some(extension) => controls.push(Control::RuleExtension {
                            extension: extension.to_owned(),
                        }),
                        None => (),
                    },
                    b"Rule-Color-Space" => match get_attributes(ev.attributes())?.get("name") {
                        Some(name) => controls.push(Control::RuleColorSpace {
                            name: name.to_owned(),
                        }),
                        None => (),
                    },
                    b"Rule-Color-Profile" => match get_attributes(ev.attributes())?.get("name") {
                        Some(name) => controls.push(Control::RuleColorProfile {
                            name: name.to_owned(),
                        }),
                        None => (),
                    },
                    b"Rule-Min-Values" => match get_attributes(ev.attributes())?.get("min") {
                        Some(min) => match min.parse() {
                            Ok(min) => controls.push(Control::RuleMinValues { min: min }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Max-Values" => match get_attributes(ev.attributes())?.get("max") {
                        Some(max) => match max.parse() {
                            Ok(max) => controls.push(Control::RuleMaxValues { max: max }),
                            Err(_) => (),
                        },
                        None => (),
                    },
                    b"Rule-Must-Be-Greater-Than-Another-Field" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => {
                                controls.push(Control::RuleMustBeGreaterThanAnotherField {
                                    field: field.to_owned(),
                                })
                            }
                            None => (),
                        }
                    }
                    b"Rule-Must-Be-Greater-Than-Or-Equal-Another-Field" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => {
                                controls.push(Control::RuleMustBeGreaterThanOrEqualAnotherField {
                                    field: field.to_owned(),
                                })
                            }
                            None => (),
                        }
                    }
                    b"Rule-Must-Be-Less-Than-Another-Field" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => controls.push(Control::RuleMustBeLessThanAnotherField {
                                field: field.to_owned(),
                            }),
                            None => (),
                        }
                    }
                    b"Rule-Must-Be-Less-Than-Or-Equal-Another-Field" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => {
                                controls.push(Control::RuleMustBeLessThanOrEqualAnotherField {
                                    field: field.to_owned(),
                                })
                            }
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Is-Not-Empty" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => {
                                controls.push(Control::RuleRequiredIfAnotherFieldIsNotEmpty {
                                    field: field.to_owned(),
                                })
                            }
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Has-Options" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => {
                                let mut options: Vec<OptionRule> = vec![];
                                loop {
                                    match reader.read_event_into(buf)? {
                                        Event::Start(ev) | Event::Empty(ev) => {
                                            match ev.name().as_ref() {
                                                b"Option" => {
                                                    match get_attributes(ev.attributes())?
                                                        .get("key")
                                                    {
                                                        Some(key) => options.push(OptionRule {
                                                            key: key.to_owned(),
                                                        }),
                                                        None => (),
                                                    }
                                                }
                                                _ => (),
                                            }
                                        }
                                        Event::End(ev) => break,
                                        _ => (),
                                    }
                                }
                                buf.clear();
                                if options.len() > 0 {
                                    controls.push(Control::RuleRequiredIfAnotherFieldHasOptions {
                                        field: field.to_owned(),
                                        options: options,
                                    })
                                }
                            }
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Is-Greater-Than" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => match get_attributes(ev.attributes())?.get("value") {
                                Some(value) => controls.push(
                                    Control::RuleRequiredIfAnotherFieldIsGreaterThan {
                                        field: field.to_owned(),
                                        value: value.to_owned(),
                                    },
                                ),
                                None => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Is-Greater-Than-Or-Equal" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => match get_attributes(ev.attributes())?.get("value") {
                                Some(value) => controls.push(
                                    Control::RuleRequiredIfAnotherFieldIsGreaterThanOrEqual {
                                        field: field.to_owned(),
                                        value: value.to_owned(),
                                    },
                                ),
                                None => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Is-Less-Than" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => match get_attributes(ev.attributes())?.get("value") {
                                Some(value) => {
                                    controls.push(Control::RuleRequiredIfAnotherFieldIsLessThan {
                                        field: field.to_owned(),
                                        value: value.to_owned(),
                                    })
                                }
                                None => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Is-Less-Than-Or-Equal" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => match get_attributes(ev.attributes())?.get("value") {
                                Some(value) => controls.push(
                                    Control::RuleRequiredIfAnotherFieldIsLessThanOrEqual {
                                        field: field.to_owned(),
                                        value: value.to_owned(),
                                    },
                                ),
                                None => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Required-If-Another-Field-Is-Equal-To" => {
                        match get_attributes(ev.attributes())?.get("field") {
                            Some(field) => match get_attributes(ev.attributes())?.get("value") {
                                Some(value) => {
                                    controls.push(Control::RuleRequiredIfAnotherFieldIsEqualTo {
                                        field: field.to_owned(),
                                        value: value.to_owned(),
                                    })
                                }
                                None => (),
                            },
                            None => (),
                        }
                    }
                    b"Rule-Condition" => {
                        match self.process_rule_condition(
                            get_attributes(ev.attributes())?,
                            reader,
                            buf,
                        )? {
                            Some(control) => controls.push(control),
                            None => (),
                        }
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Identifier" => break,
                    b"Classification" => break,
                    b"Field" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(controls)
    }
    fn process_rule_condition(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Control>, Box<dyn Error>> {
        let mut key: String = "UNKNOWN".to_owned();
        match attributes.get("key") {
            Some(key_s) => key = key_s.to_owned(),
            None => (),
        }
        let mut control = Control::RuleCondition {
            key: key,
            condition_groups: vec![],
            title: "UNKNOWN".to_owned(),
            title_locals: None,
        };
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Title" => control.set_title(self.handle_text(reader, buf)?),
                    b"Title-Local" => {
                        match get_attributes(ev.attributes())?.get("lang") {
                            Some(lang) => control
                                .add_title_local(self.handle_optional_local(lang, reader, buf)?),
                            None => (), // Ignore if there is no lang attribute
                        }
                    }
                    b"Conditions" => {
                        control.add_condition_groups(
                            self.process_rule_condition_conditions(reader, buf)?,
                        );
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Rule-Condition" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }

        Ok(Some(control))
    }
    fn process_rule_condition_conditions(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<ConditionGroup>, Box<dyn Error>> {
        let mut condition_groups: Vec<ConditionGroup> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Condition-Group" => {
                        match self.process_rule_condition_condition_group(reader, buf)? {
                            Some(condition_group) => condition_groups.push(condition_group),
                            None => (),
                        }
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Conditions" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(condition_groups)
    }
    fn process_rule_condition_condition_group(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<ConditionGroup>, Box<dyn Error>> {
        let mut conditions: Vec<Condition> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) | Event::Empty(ev) => match ev.name().as_ref() {
                    b"Condition" => match self.process_rule_condition_condition_group_condition(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )? {
                        Some(condition) => conditions.push(condition),
                        None => (),
                    },
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Condition-Group" => break,
                    _ => (),
                },
                _ => (),
            }
        }
        Ok(Some(ConditionGroup {
            conditions: conditions,
        }))
    }
    fn process_rule_condition_condition_group_condition(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Condition>, Box<dyn Error>> {
        match attributes.get("source") {
            Some(source) => match attributes.get("operator") {
                Some(operator) => match operator.as_bytes() {
                    b"EMPTY" => Ok(Some(Condition::Empty {
                        source: source.to_owned(),
                    })),
                    b"NOT_EMPTY" => Ok(Some(Condition::NotEmpty {
                        source: source.to_owned(),
                    })),
                    b"CONTAINS" => match self.handle_value_text(reader, buf)? {
                        Some(value) => Ok(Some(Condition::Contains {
                            source: source.to_owned(),
                            value: value.to_owned(),
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_CONTAINS_EMPTY_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator CONTAINS and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"EQUALS" => match self.handle_value_text(reader, buf)? {
                        Some(value) => match attributes.get("use") {
                            Some(use_suffix) => match use_suffix.as_bytes() {
                                b"SUFFIX" => Ok(Some(Condition::Equals {
                                    source: source.to_owned(),
                                    value: value.to_owned(),
                                    use_suffix: Some(UseSuffix::Suffix),
                                })),
                                b"SUFFIX_KEY" => Ok(Some(Condition::Equals {
                                    source: source.to_owned(),
                                    value: value.to_owned(),
                                    use_suffix: Some(UseSuffix::SuffixKey),
                                })),
                                b"VALUE" => Ok(Some(Condition::Equals {
                                    source: source.to_owned(),
                                    value: value.to_owned(),
                                    use_suffix: Some(UseSuffix::Value),
                                })),
                                name => {
                                    self.add_warning(Log {
                                        code: "UNKNOWN_USE_VALUE".to_owned(),
                                        message: format!("A condition with @source='{}' and @operator='EQUALS' has been found with @use='{}' that does not match SUFFIX | SUFFIX_KEY | VALUE. This condition has been ignored.", source, from_utf8(name)?),
                                    });
                                    Ok(None)
                                }
                            },
                            None => Ok(Some(Condition::Equals {
                                source: source.to_owned(),
                                value: value.to_owned(),
                                use_suffix: None,
                            })),
                        },
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_EQUALS_EMPTY_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator EQUALS and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"STARTS_WITH" => match self.handle_value_text(reader, buf)? {
                        Some(value) => Ok(Some(Condition::StartsWith {
                            source: source.to_owned(),
                            value: value.to_owned(),
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_STARTS_WITH_EMPTY_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator STARTS_WITH and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"ENDS_WITH" => match self.handle_value_text(reader, buf)? {
                        Some(value) => Ok(Some(Condition::EndsWith {
                            source: source.to_owned(),
                            value: value.to_owned(),
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_ENDS_WITH_EMPTY_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator ENDS_WITH and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"GREATER_THAN" => match self.handle_value_number(reader, buf)? {
                        Some(value) => Ok(Some(Condition::GreaterThan {
                            source: source.to_owned(),
                            value: value,
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_GREATER_THAN_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator GREATER_THAN and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"GREATER_THAN_OR_EQUAL" => match self.handle_value_number(reader, buf)? {
                        Some(value) => Ok(Some(Condition::GreaterThanOrEqual {
                            source: source.to_owned(),
                            value: value,
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_GREATER_THAN_OR_EQUAL_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator GREATER_THAN_OR_EQUAL and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"LESS_THAN" => match self.handle_value_number(reader, buf)? {
                        Some(value) => Ok(Some(Condition::LessThan {
                            source: source.to_owned(),
                            value: value,
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_LESS_THAN_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator LESS_THAN and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"LESS_THAN_OR_EQUAL" => match self.handle_value_number(reader, buf)? {
                        Some(value) => Ok(Some(Condition::LessThanOrEqual {
                            source: source.to_owned(),
                            value: value,
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_LESS_THAN_OR_EQUAL_VALUE_ATTRIBUTE".to_owned(),
                                    message: format!("A condition with operator LESS_THAN_OR_EQUAL and @source='{}' has been found without value attribute. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"IN" => match self.handle_values(reader, buf)? {
                        Some(values) => Ok(Some(Condition::In {
                            source: source.to_owned(),
                            values: values,
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_IN_MISSING_VALUES".to_owned(),
                                    message: format!("A condition with operator IN and @source='{}' has been found without Value attributes. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    b"NOT_IN" => match self.handle_values(reader, buf)? {
                        Some(values) => Ok(Some(Condition::In {
                            source: source.to_owned(),
                            values: values,
                        })),
                        None => {
                            self.add_warning(Log {
                                    code: "CONDITION_NOT_IN_MISSING_VALUES".to_owned(),
                                    message: format!("A condition with operator NOT_IN and @source='{}' has been found without Value attributes. This condition has been ignored.", source)
                                });
                            Ok(None)
                        }
                    },
                    name => {
                        self.add_warning(Log {
                            code: "UNKNOWN_RULE_CONDITION_OPERATOR".to_owned(),
                            message: format!("Unknown operator '{}' has been found in a Rule-Condition > Condition. This condition has not been imported.", from_utf8(name)?)
                        });
                        Ok(None)
                    }
                },
                None => {
                    self.add_warning(Log {
                        code: "CONDITION_EMPTY_OPERATOR".to_owned(),
                        message: format!("A condition with @source='{}' has been found without operator attribute. This condition has been ignored.", source)
                    });
                    Ok(None)
                }
            },
            None => {
                self.add_warning(Log {
                    code: "CONDITION_EMPTY_SOURCE".to_owned(),
                    message: "A condition without source attribute has been found. This condition has been ignored.".to_owned()
                });
                Ok(None)
            }
        }
    }

    fn process_sections(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Section" => {
                        self.process_section(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Sections" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_section(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut section = Section::new();
        match attributes.get("key") {
            Some(key) => section.key = key.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => section.title = self.handle_text(reader, buf)?,
                        b"Description" => {
                            section.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => section.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => section.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => section
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Section" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.sections.push(section);

        Ok(())
    }

    fn process_screens(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Screen" => {
                        self.process_screen(get_attributes(ev.attributes())?, reader, buf)?
                    }
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Screens" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }
    fn process_screen(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut screen = Screen::new();
        match attributes.get("key") {
            Some(key) => screen.key = key.to_owned(),
            None => (),
        }
        match attributes.get("level") {
            Some(level) => screen.level = level.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => screen.title = self.handle_text(reader, buf)?,
                        b"Position" => screen.position = self.handle_number(reader, buf)?,
                        b"Description" => {
                            screen.description = self.handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => screen.add_title_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => screen.add_description_local(
                                    self.handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => screen
                                    .add_metadata(self.handle_optional_metadata(key, reader, buf)?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Grid" => self.process_screen_grid(&mut screen, reader, buf)?,
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Screen" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        self.table.schema.screens.push(screen);

        Ok(())
    }
    fn process_screen_grid(
        &mut self,
        screen: &mut Screen,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Line-Height" => screen.grid.line_height = self.handle_text(reader, buf)?,
                    b"Common" => screen.grid.common = self.process_grid_common(reader, buf)?,
                    b"Specific" => screen.grid.specifics.push(self.process_grid_specific(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?),
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Grid" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }

        Ok(())
    }
    fn process_grid_common(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Vec<CommonSection>, Box<dyn Error>> {
        let mut common: Vec<CommonSection> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Section" => common.push(self.process_grid_common_section(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?),
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Common" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }
        Ok(common)
    }
    fn process_grid_common_section(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<CommonSection, Box<dyn Error>> {
        let mut section: CommonSection = CommonSection::new();
        match attributes.get("key") {
            Some(key) => section.key = key.to_owned(),
            None => (),
        }
        match attributes.get("position") {
            Some(position) => section.position = position.parse().unwrap_or(0),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) | Event::Empty(ev) => {
                    let attributes = get_attributes(ev.attributes())?;
                    let key = attributes
                        .get("key")
                        .unwrap_or(&"UNKNWON".to_owned())
                        .to_owned();
                    let position = attributes
                        .get("position")
                        .unwrap_or(&"0".to_owned())
                        .parse()
                        .unwrap_or(0);
                    let width = attributes.get("width").cloned();
                    let read_only = match attributes.get("read-only") {
                        Some(read_only) => match read_only.as_str() {
                            "true" => Some(true),
                            _ => None,
                        },
                        None => None,
                    };
                    let fixed = match attributes.get("fixed") {
                        Some(fixed) => match fixed.as_str() {
                            "true" => Some(true),
                            _ => None,
                        },
                        None => None,
                    };
                    match ev.name().as_ref() {
                        b"Column-Identifier" => {
                            section.columns.push(CommonColumn::ColumnIdentifier(Column {
                                key: key.to_owned(),
                                position: position,
                                width: width,
                                read_only: read_only,
                                fixed: fixed,
                            }))
                        }
                        b"Column-Classification" => {
                            section
                                .columns
                                .push(CommonColumn::ColumnClassification(Column {
                                    key: key.to_owned(),
                                    position: position,
                                    width: width,
                                    read_only: read_only,
                                    fixed: fixed,
                                }))
                        }
                        b"Column-Field" => {
                            section.columns.push(CommonColumn::ColumnField(Column {
                                key: key.to_owned(),
                                position: position,
                                width: width,
                                read_only: read_only,
                                fixed: fixed,
                            }))
                        }
                        b"Column-Conditional-Formatting" => {
                            section
                                .columns
                                .push(CommonColumn::ColumnConditionalFormatting(Column {
                                    key: key.to_owned(),
                                    position: position,
                                    width: width,
                                    read_only: read_only,
                                    fixed: fixed,
                                }))
                        }
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Section" => break,
                    b"Column-Identifier" => break,
                    b"Column-Classification" => break,
                    b"Column-Conditional-Formatting" => break,
                    b"Column-Field" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }

        Ok(section)
    }
    fn process_grid_specific(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<GridSpecific, Box<dyn Error>> {
        let mut specific: GridSpecific = GridSpecific::new();
        match attributes.get("classification") {
            Some(classification) => specific.classification = classification.to_owned(),
            None => (),
        }
        match attributes.get("category") {
            Some(category) => specific.category = category.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Section" => specific.sections.push(self.process_grid_specific_section(
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?),
                    _ => (),
                },
                Event::End(ev) => match ev.name().as_ref() {
                    b"Specific" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }

        Ok(specific)
    }
    fn process_grid_specific_section(
        &mut self,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<SpecificSection, Box<dyn Error>> {
        let mut section: SpecificSection = SpecificSection::new();
        match attributes.get("key") {
            Some(key) => section.key = key.to_owned(),
            None => (),
        }
        match attributes.get("position") {
            Some(position) => section.position = position.parse().unwrap_or(0),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) | Event::Empty(ev) => {
                    let attributes = get_attributes(ev.attributes())?;
                    let key = attributes
                        .get("key")
                        .unwrap_or(&"UNKNWON".to_owned())
                        .to_owned();
                    let position = attributes
                        .get("position")
                        .unwrap_or(&"0".to_owned())
                        .parse()
                        .unwrap_or(0);
                    let width = attributes.get("width").cloned();
                    let read_only = match attributes.get("read-only") {
                        Some(read_only) => match read_only.as_str() {
                            "true" => Some(true),
                            _ => None,
                        },
                        None => None,
                    };
                    let fixed = match attributes.get("fixed") {
                        Some(fixed) => match fixed.as_str() {
                            "true" => Some(true),
                            _ => None,
                        },
                        None => None,
                    };
                    match ev.name().as_ref() {
                        b"Column-Field" => section.columns.push(Column {
                            key: key.to_owned(),
                            position: position,
                            width: width,
                            read_only: read_only,
                            fixed: fixed,
                        }),
                        _ => (),
                    }
                }
                Event::End(ev) => match ev.name().as_ref() {
                    b"Section" => break,
                    b"Column-Field" => break,
                    _ => (),
                },
                _ => (),
            }
            buf.clear();
        }

        Ok(section)
    }

    // Generic functions to handle text, number... private to this context
    fn handle_text(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let mut text: String = "UNKNOWN".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => text = ev.unescape()?.into_owned(),
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        Ok(text)
    }
    fn handle_optional_text(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let mut text: String = "".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => text = ev.unescape()?.into_owned(),
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        if text.trim() == "" {
            // Ignore description with empty string
            Ok(None)
        } else {
            Ok(Some(text))
        }
    }
    fn handle_optional_number(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<usize>, Box<dyn Error>> {
        let mut number: Option<usize> = None;
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => match ev.unescape()?.parse() {
                    Ok(value) => number = Some(value),
                    Err(_) => number = None,
                },
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        Ok(number)
    }
    fn handle_optional_local(
        &mut self,
        lang: &String,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Local>, Box<dyn Error>> {
        let mut value: String = "".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => value = ev.unescape()?.into_owned(),
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        if lang.trim() == "" || value.trim() == "" {
            Ok(None)
        } else {
            Ok(Some(Local {
                lang: lang.to_owned(),
                value: value,
            }))
        }
    }
    fn handle_optional_metadata(
        &mut self,
        key: &String,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Metadata>, Box<dyn Error>> {
        let mut value: String = "".to_owned();
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => value = ev.unescape()?.into_owned(),
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        if key.trim() == "" || value.trim() == "" {
            Ok(None)
        } else {
            Ok(Some(Metadata {
                key: key.to_owned(),
                value: value,
            }))
        }
    }
    fn handle_number(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<usize, Box<dyn Error>> {
        let mut number: usize = 0;
        loop {
            match reader.read_event_into(buf)? {
                Event::Text(ev) => number = ev.unescape()?.parse().unwrap_or(0),
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        Ok(number)
    }
    fn handle_value_text(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let mut value: Option<String> = None;
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Value" => loop {
                        match reader.read_event_into(buf)? {
                            Event::Text(ev) => value = Some(ev.unescape()?.into_owned()),
                            Event::End(_) => break,
                            _ => (),
                        }
                        buf.clear();
                    },
                    name => {
                        self.add_warning(Log {
                            code: "UNEXPECTED_ELEMENT_NAME".to_owned(),
                            message: format!(
                                "Unexpected element name found in a Value element of a Condition: '{}'.",
                                from_utf8(&name)?
                            ),
                        });
                        break;
                    }
                },
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        Ok(value)
    }
    fn handle_value_number(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<usize>, Box<dyn Error>> {
        let mut value: Option<usize> = None;
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Value" => loop {
                        match reader.read_event_into(buf)? {
                            Event::Text(ev) => match ev.unescape()?.into_owned().parse() {
                                Ok(v) => value = Some(v),
                                Err(_) => self.add_warning(Log {
                                    code: "VALUE_INVALID_DATA_TYPE".to_owned(),
                                    message: "In an element Condition > Value the value expected is a number but the value found is not a number.".to_owned(),
                                }),
                            },
                            Event::End(_) => break,
                            _ => (),
                        }
                        buf.clear();
                    },
                    _ => (),
                },
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        Ok(value)
    }
    fn handle_values(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<Option<Vec<String>>, Box<dyn Error>> {
        let mut values: Vec<String> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Value" => loop {
                        match reader.read_event_into(buf)? {
                            Event::Text(ev) => values.push(ev.unescape()?.into_owned()),
                            Event::End(_) => break,
                            _ => (),
                        }
                        buf.clear();
                    },
                    name => {
                        self.add_warning(Log {
                            code: "UNEXPECTED_ELEMENT_NAME".to_owned(),
                            message: format!(
                                "Unexpected element name found in a Value element of a Condition: '{}'.",
                                from_utf8(&name)?
                            ),
                        });
                        break;
                    }
                },
                Event::End(_) => break,
                _ => (),
            }
            buf.clear();
        }
        if values.len() > 0 {
            Ok(Some(values))
        } else {
            Ok(None)
        }
    }
    fn add_warning(&mut self, log: Log) {
        self.warnings.push(log)
    }
}
