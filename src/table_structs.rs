use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub key: String,
    pub title: String,
    pub position: usize,
    pub color: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub schema: Schema,
}
impl Table {
    pub fn new() -> Self {
        Table {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            position: 0,
            color: "BLUE".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            schema: Schema {
                partitions: vec![],
                levels: vec![],
                identifiers: vec![],
                classifications: vec![],
                fields: vec![],
                formulas: vec![],
                matrix: Matrix {
                    common: vec![],
                    specifics: vec![],
                },
                conditional_formattings: vec![],
                sections: vec![],
                screens: vec![],
            },
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
}

// Schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub partitions: Vec<Partition>,
    pub levels: Vec<Level>,
    pub identifiers: Vec<Identifier>,
    pub classifications: Vec<Classification>,
    pub fields: Vec<Field>,
    pub formulas: Vec<Formula>,
    pub matrix: Matrix,
    pub conditional_formattings: Vec<ConditionalFormatting>,
    pub sections: Vec<Section>,
    pub screens: Vec<Screen>,
}

// Partition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Partition {
    pub key: String,
    pub title: String,
    pub position: usize,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl Partition {
    pub fn new() -> Self {
        Partition {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            position: 0,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}

// Level
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub key: String,
    pub index: usize,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl Level {
    pub fn new() -> Self {
        Level {
            key: "UNKNOWN".to_owned(),
            index: 0,
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}

// Identifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub key: String,
    pub index: usize,
    pub level: String,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl Identifier {
    pub fn new() -> Self {
        Identifier {
            key: "UNKNOWN".to_owned(),
            index: 0,
            level: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}

// Classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Classification {
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
    pub categories: Vec<Category>,
}
impl Classification {
    pub fn new() -> Self {
        Classification {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            categories: vec![],
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
// Category
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub key: String,
    pub parent: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl Category {
    pub fn new() -> Self {
        Category {
            key: "UNKNOWN".to_owned(),
            parent: None,
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}

// Field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    SingleLineText,
    LongText,
    HtmlText,
    Number,
    SingleSelect,
    MultipleSelect,
    MultipleSelectQuantified,
    MultipleSelectQuantifiedWithComments,
    Date,
    DateTime,
    Image,
    Attachment,
}
impl DataType {
    pub fn to_string(&self) -> String {
        match self {
            DataType::SingleLineText => "SINGLE-LINE-TEXT".to_owned(),
            DataType::LongText => "LONG-TEXT".to_owned(),
            DataType::HtmlText => "HTML-TEXT".to_owned(),
            DataType::Number => "NUMBER".to_owned(),
            DataType::SingleSelect => "SINGLE-SELECT".to_owned(),
            DataType::MultipleSelect => "MULTIPLE-SELECT".to_owned(),
            DataType::MultipleSelectQuantified => "MULTIPLE-SELECT-QUANTIFIED".to_owned(),
            DataType::MultipleSelectQuantifiedWithComments => "MULTIPLE-SELECT-QUANTIFIED-WITH-COMMENTS".to_owned(),
            DataType::Date => "DATE".to_owned(),
            DataType::DateTime => "DATE-TIME".to_owned(),
            DataType::Image => "IMAGE".to_owned(),
            DataType::Attachment => "ATTACHMENT".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub key: String,
    pub level: String,
    pub data_type: DataType,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub precision: Option<usize>,
    pub suffixes: Option<Vec<Suffix>>,
    pub options: Option<Vec<SelectOption>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub color: Option<String>,
    pub metadata: Option<Vec<Metadata>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suffix {
    pub key: String,
    pub title: String,
    pub default: Option<bool>,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl Field {
    pub fn new(key: String, level: String, data_type: DataType) -> Self {
        Field {
            key: key,
            data_type: data_type,
            level: level,
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            prefix: None,
            suffix: None,
            precision: None,
            suffixes: None,
            options: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
impl SelectOption {
    pub fn new() -> Self {
        SelectOption {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            color: None,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
impl Suffix {
    pub fn new() -> Self {
        Suffix {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            default: None,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Matrix {
    pub common: Vec<MatrixField>,
    pub specifics: Vec<MatrixSpecific>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixField {
    pub key: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixSpecific {
    pub classification: String,
    pub category: String,
    pub fields: Vec<MatrixField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalFormatting {
    pub key: String,
    pub level: String,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
    pub default_status: DefaultStatus,
    pub statuses: Vec<Status>,
}
impl ConditionalFormatting {
    pub fn new() -> Self {
        ConditionalFormatting {
            key: "UNKNOWN".to_owned(),
            level: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            default_status: DefaultStatus::new(),
            statuses: vec![],
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultStatus {
    pub key: String,
    pub title: String,
    pub color: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl DefaultStatus {
    pub fn new() -> Self {
        DefaultStatus {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            color: "NONE".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
    pub color: String,
    pub priority: usize,
    pub rules: Rules,
}
impl Status {
    pub fn new() -> Self {
        Status {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            color: "NONE".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            priority: 0,
            rules: Rules {
                common: vec![],
                specifics: vec![],
            },
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rules {
    pub common: Vec<CommonAttributeRules>,
    pub specifics: Vec<SpecificRules>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeType {
    Identifier,
    Classification,
    Field,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonAttributeRules {
    pub attribute_type: AttributeType,
    pub key: String,
    pub controls: Vec<Control>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificRules {
    pub classification: String,
    pub category: String,
    pub attributes: Vec<SpecificAttributeRules>,
}
impl SpecificRules {
    pub fn new(
        classification: String,
        category: String,
        attributes: Vec<SpecificAttributeRules>,
    ) -> Self {
        SpecificRules {
            classification: classification,
            category: category,
            attributes: attributes,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificAttributeRules {
    pub key: String,
    pub controls: Vec<Control>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Control {
    RuleRequired,
    RuleIsLeaf,
    RuleMinLength {
        min: usize,
    },
    RuleMaxLength {
        max: usize,
    },
    RuleRegex {
        regex: String,
    },
    RuleBarcode {
        barcode_type: String,
    },
    RuleLessThan {
        value: usize,
    },
    RuleGreaterThan {
        value: usize,
    },
    RuleLessThanOrEqual {
        value: usize,
    },
    RuleGreaterThanOrEqual {
        value: usize,
    },
    RuleDecimalPlaces {
        precision: usize,
    },
    RuleMinWidthPx {
        min: usize,
    },
    RuleMaxWidthPx {
        max: usize,
    },
    RuleMinHeightPx {
        min: usize,
    },
    RuleMaxHeightPx {
        max: usize,
    },
    RuleMaxSizeKb {
        max: usize,
    },
    RuleExtension {
        extension: String,
    },
    RuleColorSpace {
        name: String,
    },
    RuleColorProfile {
        name: String,
    },
    RuleMinValues {
        min: usize,
    },
    RuleMaxValues {
        max: usize,
    },
    // Inter fields
    RuleMustBeGreaterThanAnotherField {
        field: String,
    },
    RuleMustBeGreaterThanOrEqualAnotherField {
        field: String,
    },
    RuleMustBeLessThanAnotherField {
        field: String,
    },
    RuleMustBeLessThanOrEqualAnotherField {
        field: String,
    },
    RuleRequiredIfAnotherFieldIsNotEmpty {
        field: String,
    },
    RuleRequiredIfAnotherFieldHasOptions {
        field: String,
        options: Vec<OptionRule>,
    },
    RuleRequiredIfAnotherFieldIsGreaterThan {
        field: String,
        value: String,
    },
    RuleRequiredIfAnotherFieldIsGreaterThanOrEqual {
        field: String,
        value: String,
    },
    RuleRequiredIfAnotherFieldIsLessThan {
        field: String,
        value: String,
    },
    RuleRequiredIfAnotherFieldIsLessThanOrEqual {
        field: String,
        value: String,
    },
    RuleRequiredIfAnotherFieldIsEqualTo {
        field: String,
        value: String,
    },
    RuleCondition {
        key: String,
        condition_groups: Vec<ConditionGroup>,
        title: String,
        title_locals: Option<Vec<Local>>,
    },
}
impl Control {
    pub fn set_title(&mut self, value: String) {
        if let Control::RuleCondition {
            key: _,
            condition_groups: _,
            ref mut title,
            title_locals: _,
        } = self
        {
            *title = value;
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        if let Control::RuleCondition {
            key: _,
            condition_groups: _,
            title: _,
            ref mut title_locals,
        } = self
        {
            match local {
                Some(local) => match title_locals {
                    Some(title_locals) => title_locals.push(local),
                    None => *title_locals = Some(vec![local]),
                },
                None => (),
            }
        }
    }
    pub fn add_condition_groups(&mut self, conditions: Vec<ConditionGroup>) {
        if let Control::RuleCondition {
            key: _,
            ref mut condition_groups,
            title: _,
            title_locals: _,
        } = self
        {
            *condition_groups = conditions;
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionRule {
    pub key: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionGroup {
    pub conditions: Vec<Condition>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    Empty {
        source: String,
    },
    NotEmpty {
        source: String,
    },
    Contains {
        source: String,
        value: String,
    },
    Equals {
        source: String,
        value: String,
        use_suffix: Option<UseSuffix>,
    },
    NotEquals {
        source: String,
        value: String,
    },
    StartsWith {
        source: String,
        value: String,
    },
    EndsWith {
        source: String,
        value: String,
    },
    LessThan {
        source: String,
        value: usize,
    },
    LessThanOrEqual {
        source: String,
        value: usize,
    },
    GreaterThan {
        source: String,
        value: usize,
    },
    GreaterThanOrEqual {
        source: String,
        value: usize,
    },
    In {
        source: String,
        values: Vec<String>,
    },
    NotIn {
        source: String,
        values: Vec<String>,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UseSuffix {
    Value,
    SuffixKey,
    Suffix,
}
impl UseSuffix {
    pub fn to_string(&self) -> String {
        match self {
            UseSuffix::Value => "VALUE".to_owned(),
            UseSuffix::SuffixKey => "SUFFIX_KEY".to_owned(),
            UseSuffix::Suffix => "SUFFIX".to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Formula {
    pub attribute_type: AttributeType,
    pub key: String,
    pub rules: Vec<Rule>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub priority: usize,
    pub conditions: Vec<ConditionGroup>,
    pub action: Action,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    SetTextTemplate {
        trim_spaces: bool,
        value: String,
    },
    SetNumberTemplate {
        precision: usize,
        round: String,
        value: String,
    },
    SetSelectableOptions {
        values: Vec<String>,
    },
}
impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::SetTextTemplate { trim_spaces, value } => "SET_TEXT".to_owned(),
            Action::SetNumberTemplate { precision, round, value } => "SET_NUMBER".to_owned(),
            Action::SetSelectableOptions { values } => "SET_SELECTABLE_OPTIONS".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
}
impl Section {
    pub fn new() -> Self {
        Section {
            key: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Screen {
    pub key: String,
    pub level: String,
    pub position: usize,
    pub title: String,
    pub description: Option<String>,
    pub title_locals: Option<Vec<Local>>,
    pub description_locals: Option<Vec<Local>>,
    pub metadata: Option<Vec<Metadata>>,
    pub grid: ScreenGrid,
}
impl Screen {
    pub fn new() -> Self {
        Screen {
            key: "UNKNOWN".to_owned(),
            level: "UNKNOWN".to_owned(),
            title: "UNKNOWN".to_owned(),
            position: 0,
            description: None,
            title_locals: None,
            description_locals: None,
            metadata: None,
            grid: ScreenGrid {
                line_height: "MEDIUM".to_owned(),
                common: vec![],
                specifics: vec![],
            },
        }
    }
    pub fn add_title_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.title_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.title_locals = Some(new_locals)
                }
                None => self.title_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_description_local(&mut self, local: Option<Local>) {
        match local {
            Some(local) => match self.description_locals.as_ref() {
                Some(locals) => {
                    let mut new_locals = locals.to_vec();
                    new_locals.push(local);
                    self.description_locals = Some(new_locals)
                }
                None => self.description_locals = Some(vec![local]),
            },
            None => (),
        }
    }
    pub fn add_metadata(&mut self, metadata: Option<Metadata>) {
        match metadata {
            Some(metadata) => match self.metadata.as_ref() {
                Some(metadatas) => {
                    let mut new_mertadatas = metadatas.to_vec();
                    new_mertadatas.push(metadata);
                    self.metadata = Some(new_mertadatas)
                }
                None => self.metadata = Some(vec![metadata]),
            },
            None => (),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenGrid {
    pub line_height: String,
    pub common: Vec<CommonSection>,
    pub specifics: Vec<GridSpecific>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonSection {
    pub key: String,
    pub position: usize,
    pub columns: Vec<CommonColumn>,
}
impl CommonSection {
    pub fn new() -> Self {
        CommonSection {
            key: "UNKNOWN".to_owned(),
            position: 0,
            columns: vec![],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommonColumn {
    ColumnIdentifier(Column),
    ColumnClassification(Column),
    ColumnConditionalFormatting(Column),
    ColumnField(Column),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridSpecific {
    pub classification: String,
    pub category: String,
    pub sections: Vec<SpecificSection>,
}
impl GridSpecific {
    pub fn new() -> Self {
        GridSpecific {
            classification: "UNKNOWN".to_owned(),
            category: "UNKNOWN".to_owned(),
            sections: vec![],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificSection {
    pub key: String,
    pub position: u32,
    pub columns: Vec<Column>,
}
impl SpecificSection {
    pub fn new() -> Self {
        SpecificSection {
            key: "UNKNOWN".to_owned(),
            position: 0,
            columns: vec![],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub key: String,
    pub position: u32,
    pub width: Option<String>,
    pub read_only: Option<bool>,
    pub fixed: Option<bool>,
}

// Utils
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Local {
    pub lang: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub key: String,
    pub value: String,
}
