use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Write};

use crate::table_structs::Table;

pub struct TableXmlWriter {}

impl TableXmlWriter {
    pub fn write(table: &Table, path: &str) -> Result<(), Box<dyn Error>> {
        let mut buffer = File::create(path)?;
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut table_el = BytesStart::new("Table");
        table_el.push_attribute(("key", table.key.as_str()));
        writer.write_event(Event::Start(table_el))?;

        writer.write_event(Event::End(BytesEnd::new("Table")))?;

        let result = writer.into_inner().into_inner();
        buffer.write(&result)?;

        Ok(())
    }
}
