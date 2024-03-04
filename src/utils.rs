use std::collections::HashMap;
use std::error::Error;
use quick_xml::events::attributes::Attributes;
use std::str;

pub fn get_attributes(attributes: Attributes<'_>) -> Result<HashMap<String,String>, Box<dyn Error>> {
    let mut formatted_attributes = HashMap::new();
    for attribute in attributes {
        let att = attribute?;
        let key: String = str::parse(str::from_utf8(att.key.0)?)?;
        let value: String = str::parse(str::from_utf8(att.value.as_ref())?)?;
        formatted_attributes.insert(key, value);
    }
    Ok(formatted_attributes)
}