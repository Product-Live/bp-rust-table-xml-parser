use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use quick_xml::{events::Event, Reader};

use crate::{
    table_structs::{
        Category, Classification, DataType, Field, Identifier, Level, Local, MatrixField,
        MatrixSpecific, Metadata, Partition, Section, SelectOption, Suffix, Table,
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
                    b"Sections" => Self::process_sections(self, reader, buf)?,
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
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Category" => Self::process_category(
                        classification,
                        get_attributes(ev.attributes())?,
                        reader,
                        buf,
                    )?,
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
        Ok(())
    }
    fn process_category(
        classification: &mut Classification,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
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
        classification.categories.push(category);
        Ok(())
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
                        _ => (),
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
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Option" => {
                        Self::process_option(field, get_attributes(ev.attributes())?, reader, buf)?
                    }
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
        Ok(())
    }
    fn process_option(
        field: &mut Field,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
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
        if field.options.is_none() {
            field.options = Some(vec![option]);
        } else {
            let mut new_options = field.options.as_ref().unwrap().to_vec();
            new_options.push(option);
            field.options = Some(new_options);
        }
        Ok(())
    }
    fn process_suffixes(
        field: &mut Field,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            match reader.read_event_into(buf)? {
                Event::Start(ev) => match ev.name().as_ref() {
                    b"Suffix" => {
                        Self::process_suffix(field, get_attributes(ev.attributes())?, reader, buf)?
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
        Ok(())
    }
    fn process_suffix(
        field: &mut Field,
        attributes: HashMap<String, String>,
        reader: &mut Reader<BufReader<File>>,
        buf: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
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
        if field.suffixes.is_none() {
            field.suffixes = Some(vec![suffix]);
        } else {
            let mut new_suffixes = field.suffixes.as_ref().unwrap().to_vec();
            new_suffixes.push(suffix);
            field.suffixes = Some(new_suffixes);
        }
        Ok(())
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
                Event::Start(ev) => match ev.name().as_ref() {
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
                Event::Empty(ev) => match ev.name().as_ref() {
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
                Event::Start(ev) => match ev.name().as_ref() {
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
                Event::Empty(ev) => match ev.name().as_ref() {
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
