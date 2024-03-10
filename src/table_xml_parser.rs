use std::{collections::HashMap, error::Error, fs::File, io::BufReader, str::from_utf8};

use quick_xml::{events::Event, Reader};

use crate::{
    table_structs::{
        AttributeType, Category, Classification, Column, CommonAttributeRules, CommonColumn,
        CommonSection, ConditionalFormatting, Control, DataType, DefaultStatus, Field,
        GridSpecific, Identifier, Level, Local, MatrixField, MatrixSpecific, Metadata, OptionRule,
        Partition, Screen, Section, SelectOption, SpecificSection, Status, Suffix, Table,
    },
    utils::get_attributes,
};

pub struct TableXmlParser {
    pub table: Table,
}

impl TableXmlParser {
    pub fn read(path: &str) -> Result<Table, Box<dyn Error>> {
        let mut xml_parser = TableXmlParser {
            table: Table::new(),
        };
        xml_parser.process_xml(path)?;
        Ok(xml_parser.table)
    }
    // Parse xml file
    fn process_xml(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut reader: Reader<BufReader<File>> = Reader::from_file(path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                // Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Event::Eof => break,

                Event::Start(ev) => match ev.name().as_ref() {
                    b"Table" => {
                        self.process_table(get_attributes(ev.attributes())?, &mut reader, &mut buf)?
                    }
                    _ => (),
                },

                _ => (),
            }
            buf.clear();
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
                        b"Title" => self.table.title = Self::handle_text(reader, buf)?,
                        b"Position" => self.table.position = Self::handle_number(reader, buf)?,
                        b"Color" => self.table.color = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            self.table.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => self.table.add_title_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => {
                                    self.table
                                        .add_description_local(Self::handle_optional_local(
                                            lang, reader, buf,
                                        )?)
                                }
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Schema" => Self::process_schema(self, reader, buf)?,
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
                    b"Partitions" => Self::process_partitions(self, reader, buf)?,
                    b"Levels" => Self::process_levels(self, reader, buf)?,
                    b"Identifiers" => Self::process_identifiers(self, reader, buf)?,
                    b"Classifications" => Self::process_classifications(self, reader, buf)?,
                    b"Fields" => Self::process_fields(self, reader, buf)?,
                    b"Matrix" => Self::process_matrix(self, reader, buf)?,
                    b"Conditional-Formattings" => {
                        Self::process_conditional_formattings(self, reader, buf)?
                    }
                    b"Sections" => Self::process_sections(self, reader, buf)?,
                    b"Screens" => Self::process_screens(self, reader, buf)?,
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
                    b"Partition" => Self::process_partition(
                        self,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
                        b"Title" => partition.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            partition.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => partition.add_title_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => partition.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => partition.add_metadata(
                                    Self::handle_optional_metadata(key, reader, buf)?,
                                ),
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
                        Self::process_level(self, get_attributes(ev.attributes())?, reader, buf)?
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
                        b"Title" => level.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            level.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => level.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => level.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => level.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
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
                    b"Identifier" => Self::process_identifier(
                        self,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
                        b"Title" => identifier.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            identifier.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => identifier.add_title_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => identifier.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => identifier.add_metadata(
                                    Self::handle_optional_metadata(key, reader, buf)?,
                                ),
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
                    b"Classification" => Self::process_classification(
                        self,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
                        b"Title" => classification.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            classification.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => classification.add_title_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => classification.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => classification.add_metadata(
                                    Self::handle_optional_metadata(key, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Categories" => {
                            Self::process_categories(&mut classification, reader, buf)?
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
        classification: &mut Classification,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut categories: Vec<Category> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Category" => categories.push(Self::process_category(
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
        classification.categories = categories;
        Ok(())
    }
    fn process_category(
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
                        b"Title" => category.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            category.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => category.add_title_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => category.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => category.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
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
                        Self::process_field(self, get_attributes(ev.attributes())?, reader, buf)?
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
                        b"Title" => field.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            field.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => field.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => field.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => field.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Prefix" => field.prefix = Self::handle_optional_text(reader, buf)?,
                        b"Suffix" => field.suffix = Self::handle_optional_text(reader, buf)?,
                        b"Precision" => {
                            field.precision = Self::handle_optional_number(reader, buf)?
                        }
                        b"Options" => Self::process_options(&mut field, reader, buf)?,
                        b"Suffixes" => Self::process_suffixes(&mut field, reader, buf)?,
                        name => Err(format!(
                            "Unknown element name '{}' within a Field element",
                            from_utf8(name).unwrap()
                        ))?,
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
        field: &mut Field,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut options: Vec<SelectOption> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Option" => options.push(Self::process_option(
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
                        b"Title" => option.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            option.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => option.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => option.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => option.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        name => Err(format!(
                            "Unknown element name '{}' within a Option element",
                            from_utf8(name).unwrap()
                        ))?,
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
        // if field.options.is_none() {
        //     field.options = Some(vec![option]);
        // } else {
        //     let mut new_options = field.options.as_ref().unwrap().to_vec();
        //     new_options.push(option);
        //     field.options = Some(new_options);
        // }
        Ok(option)
    }
    fn process_suffixes(
        field: &mut Field,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut suffixes: Vec<Suffix> = vec![];
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Suffix" => {
                        suffixes.push(Self::process_suffix(
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
                        b"Title" => suffix.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            suffix.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => suffix.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => suffix.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => suffix.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
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
                    b"Common" => Self::process_matrix_common(self, reader, buf)?,
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
                        Self::process_matrix_specific(&mut specific, reader, buf)?;
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

    fn process_conditional_formattings(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Conditional-Formatting" => Self::process_conditional_formatting(
                        self,
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
                        b"Title" => conditional_formatting.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            conditional_formatting.description =
                                Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => conditional_formatting.add_title_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => conditional_formatting.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => conditional_formatting.add_metadata(
                                    Self::handle_optional_metadata(key, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Default-Status" => Self::process_conditional_formatting_default_status(
                            &mut conditional_formatting,
                            get_attributes(ev.attributes())?,
                            reader,
                            buf,
                        )?,
                        b"Statuses" => Self::process_conditional_formatting_statuses(
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
                    b"Title" => default_status.title = Self::handle_text(reader, buf)?,
                    b"Color" => default_status.color = Self::handle_text(reader, buf)?,
                    b"Description" => {
                        default_status.description = Self::handle_optional_text(reader, buf)?
                    }
                    b"Title-Local" => {
                        match get_attributes(ev.attributes())?.get("lang") {
                            Some(lang) => default_status
                                .add_title_local(Self::handle_optional_local(lang, reader, buf)?),
                            None => (), // Ignore if there is no lang attribute
                        }
                    }
                    b"Description-Local" => {
                        match get_attributes(ev.attributes())?.get("lang") {
                            Some(lang) => default_status.add_description_local(
                                Self::handle_optional_local(lang, reader, buf)?,
                            ),
                            None => (), // Ignore if there is no lang attribute
                        }
                    }
                    b"Metadata" => {
                        match get_attributes(ev.attributes())?.get("key") {
                            Some(key) => default_status
                                .add_metadata(Self::handle_optional_metadata(key, reader, buf)?),
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
        conditional_formatting: &mut ConditionalFormatting,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Status" => Self::process_conditional_formatting_status(
                        conditional_formatting,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
        Ok(())
    }
    fn process_conditional_formatting_status(
        conditional_formatting: &mut ConditionalFormatting,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut status = Status::new();
        match attributes.get("key") {
            Some(key) => status.key = key.to_owned(),
            None => (),
        }
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => {
                    match ev.name().as_ref() {
                        b"Title" => status.title = Self::handle_text(reader, buf)?,
                        b"Color" => status.color = Self::handle_text(reader, buf)?,
                        b"Priority" => status.priority = Self::handle_number(reader, buf)?,
                        b"Description" => {
                            status.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => status.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => status.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => status.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Rules" => Self::process_status_rules(&mut status, reader, buf)?,
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
        conditional_formatting.statuses.push(status);
        Ok(())
    }
    fn process_status_rules(
        status: &mut Status,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Common" => Self::process_status_rules_common(status, reader, buf)?,
                    // b"Specfic" => Self::process_status_rules_specific(
                    //     status,
                    //     get_attributes(ev.attributes()),
                    //     reader,
                    //     buf,
                    // )?,
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
        status: &mut Status,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Identifier" | b"Classification" | b"Field" => {
                        let mut key = "UNKNOWN".to_owned();
                        match get_attributes(ev.attributes())?.get("key") {
                            Some(key_s) => key = key_s.to_owned(),
                            None => (),
                        }
                        let controls = Self::process_controls(reader, buf)?;
                        if controls.len() > 0 {
                            status.rules.common.push(CommonAttributeRules {
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
        Ok(())
    }
    fn process_controls(
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

    fn process_sections(
        &mut self,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Section" => {
                        Self::process_section(self, get_attributes(ev.attributes())?, reader, buf)?
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
                        b"Title" => section.title = Self::handle_text(reader, buf)?,
                        b"Description" => {
                            section.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => section.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => section.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => section.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
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
                        Self::process_screen(self, get_attributes(ev.attributes())?, reader, buf)?
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
                        b"Title" => screen.title = Self::handle_text(reader, buf)?,
                        b"Position" => screen.position = Self::handle_number(reader, buf)?,
                        b"Description" => {
                            screen.description = Self::handle_optional_text(reader, buf)?
                        }
                        b"Title-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => screen.add_title_local(Self::handle_optional_local(
                                    lang, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Description-Local" => {
                            match get_attributes(ev.attributes())?.get("lang") {
                                Some(lang) => screen.add_description_local(
                                    Self::handle_optional_local(lang, reader, buf)?,
                                ),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Metadata" => {
                            match get_attributes(ev.attributes())?.get("key") {
                                Some(key) => screen.add_metadata(Self::handle_optional_metadata(
                                    key, reader, buf,
                                )?),
                                None => (), // Ignore if there is no lang attribute
                            }
                        }
                        b"Grid" => Self::process_screen_grid(&mut screen, reader, buf)?,
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
        // &mut self,
        screen: &mut Screen,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Line-Height" => screen.grid.line_height = Self::handle_text(reader, buf)?,
                    b"Common" => Self::process_grid_common(screen, reader, buf)?,
                    b"Specific" => Self::process_grid_specific(
                        screen,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
        screen: &mut Screen,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Section" => Self::process_grid_common_section(
                        get_attributes(ev.attributes())?,
                        screen,
                        reader,
                        buf,
                    )?,
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
    fn process_grid_common_section(
        attributes: HashMap<String, String>,
        screen: &mut Screen,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
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
        screen.grid.common.push(section);

        Ok(())
    }
    fn process_grid_specific(
        screen: &mut Screen,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let mut specific = GridSpecific::new();
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
                    b"Section" => Self::process_grid_specific_section(
                        &mut specific,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
        screen.grid.specifics.push(specific);

        Ok(())
    }
    fn process_grid_specific_section(
        specific: &mut GridSpecific,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
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
        specific.sections.push(section);

        Ok(())
    }

    // Generic functions to handle text, number... private to this context
    fn handle_text(
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
}
