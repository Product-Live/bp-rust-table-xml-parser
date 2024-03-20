use crate::{
    table_xml_parser::{self, TableXmlParser},
    table_xml_writer::TableXmlWriter,
};

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

#[test]
fn write_file() {
    let table_xml_parser = TableXmlParser::read("./src/tests/inputs/valid.xml").unwrap();
    match TableXmlWriter::write(&table_xml_parser.table, "./src/tests/outputs/output.xml") {
        Ok(_) => {
            let new_table_xml_parser =
                TableXmlParser::read("./src/tests/outputs/output.xml").unwrap();
            assert_eq!(table_xml_parser.table, new_table_xml_parser.table);
        }
        Err(_) => assert!(false),
    }
}
