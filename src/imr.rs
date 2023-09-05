//! The Internal Model Representation used by our migration cli tool
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// A collection of all models used in the resulting application
#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct InternalModelFormat {
    /// List of all models
    pub models: Vec<Model>,
}

/// A single model i.e. database table
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    /// Name of the table
    pub name: String,

    /// List of columns of the table
    pub fields: Vec<Field>,

    /// Optional source reference to enhance error messages
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_defined_at: Option<Source>,
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

impl Hash for Model {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fields.hash(state);
        self.name.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        data.iter().for_each(|x| x.hash(state));
    }
}

/// Model's fields i.e. the table's columns
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Field {
    /// Name of the column
    pub name: String,

    /// Type of the column
    #[serde(rename = "Type")]
    pub db_type: DbType,

    /// List of annotations, constraints, etc.
    pub annotations: Vec<Annotation>,

    /// Optional source reference to enhance error messages
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_defined_at: Option<Source>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.db_type == other.db_type
            && self.annotations == other.annotations
    }
}

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.annotations.hash(state);
        self.db_type.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        data.iter().for_each(|x| x.hash(state));
    }
}

/// Location in the source code a [Model] or [Field] originates from
/// Used for better error messages in the migration tool
#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct Source {
    /// Filename of the source code of the [Model] or [Field]
    pub file: String,
    /// Line of the [Model] or [Field]
    pub line: usize,
    /// Column of the [Model] or [Field]
    pub column: usize,
}

/// All column types supported by the migration tool
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
    VarChar,
    Binary,
    Int8,
    Int16,
    Int32,
    Int64,
    #[serde(rename = "float_number")]
    Float,
    #[serde(rename = "double_number")]
    Double,
    Boolean,
    Date,
    DateTime,
    Timestamp,
    Time,
    Choices,
    Uuid,
    MacAddress,
    IpNetwork,
    BitVec,
}

/// The subset of annotations which need to be communicated with the migration tool
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
#[serde(tag = "Type", content = "Value")]
#[serde(rename_all = "snake_case")]
pub enum Annotation {
    /// Only for [DbType::Timestamp], [DbType::DateTime], [DbType::Time] and [DbType::Date].
    /// Will set the current time of the database when a row is created.
    AutoCreateTime,
    /// Only for [DbType::Timestamp], [DbType::DateTime], [DbType::Time] and [DbType::Date].
    /// Will set the current time of the database when a row is updated.
    AutoUpdateTime,
    /// AUTO_INCREMENT constraint
    AutoIncrement,
    /// A list of choices to set
    Choices(Vec<String>),
    /// DEFAULT constraint
    DefaultValue(DefaultValue),
    /// Create an index. The optional [IndexValue] can be used, to build more complex indexes.
    Index(Option<IndexValue>),
    /// Only for VARCHAR, VARBINARY. Specifies the maximum length of the column's content.
    MaxLength(i32),
    /// NOT NULL constraint
    NotNull,
    /// The annotated column will be used as primary key
    PrimaryKey,
    /// UNIQUE constraint
    Unique,
    /// Foreign Key constraint
    ForeignKey(ForeignKey),
}

impl Annotation {
    /**
    Alternative shallow equals function.

    Returns true on:
    ```rust
    use rorm_declaration::imr::Annotation;

    assert!(Annotation::MaxLength(0).eq_shallow(&Annotation::MaxLength(255)));
    ```
    */
    pub fn eq_shallow(&self, other: &Self) -> bool {
        match (self, other) {
            (Annotation::AutoCreateTime, Annotation::AutoCreateTime) => true,
            (Annotation::AutoCreateTime, _) => false,
            (Annotation::AutoUpdateTime, Annotation::AutoUpdateTime) => true,
            (Annotation::AutoUpdateTime, _) => false,
            (Annotation::AutoIncrement, Annotation::AutoIncrement) => true,
            (Annotation::AutoIncrement, _) => false,
            (Annotation::Choices(_), Annotation::Choices(_)) => true,
            (Annotation::Choices(_), _) => false,
            (Annotation::DefaultValue(_), Annotation::DefaultValue(_)) => true,
            (Annotation::DefaultValue(_), _) => false,
            (Annotation::Index(_), Annotation::Index(_)) => true,
            (Annotation::Index(_), _) => false,
            (Annotation::MaxLength(_), Annotation::MaxLength(_)) => true,
            (Annotation::MaxLength(_), _) => false,
            (Annotation::NotNull, Annotation::NotNull) => true,
            (Annotation::NotNull, _) => false,
            (Annotation::PrimaryKey, Annotation::PrimaryKey) => true,
            (Annotation::PrimaryKey, _) => false,
            (Annotation::Unique, Annotation::Unique) => true,
            (Annotation::Unique, _) => false,
            (Annotation::ForeignKey(_), Annotation::ForeignKey(_)) => true,
            (Annotation::ForeignKey(_), _) => false,
        }
    }

    /**
    Alternative shallow hash function.

    Returns true on:
    ```rust
    use rorm_declaration::imr::Annotation;

    assert_eq!(Annotation::MaxLength(0).hash_shallow(), Annotation::MaxLength(255).hash_shallow());
    ```
    */
    pub fn hash_shallow(&self) -> u64 {
        let mut state = DefaultHasher::new();
        match self {
            Annotation::AutoCreateTime => state.write_i8(0),
            Annotation::AutoUpdateTime => state.write_i8(1),
            Annotation::AutoIncrement => state.write_i8(2),
            Annotation::Choices(_) => state.write_i8(3),
            Annotation::DefaultValue(_) => state.write_i8(4),
            Annotation::Index(_) => state.write_i8(5),
            Annotation::MaxLength(_) => state.write_i8(6),
            Annotation::NotNull => state.write_i8(7),
            Annotation::PrimaryKey => state.write_i8(8),
            Annotation::Unique => state.write_i8(9),
            Annotation::ForeignKey(_) => state.write_i8(10),
        }
        state.finish()
    }
}

#[cfg(test)]
mod test {

    use crate::imr::{Annotation, IndexValue};

    #[test]
    fn test_annotation_hash() {
        assert_eq!(
            Annotation::MaxLength(1).hash_shallow(),
            Annotation::MaxLength(12313).hash_shallow()
        );

        assert_eq!(
            Annotation::Index(None).hash_shallow(),
            Annotation::Index(Some(IndexValue {
                priority: None,
                name: "foo".to_string(),
            }))
            .hash_shallow()
        );
    }

    #[test]
    fn test_annotation_partial_eq() {
        assert!(Annotation::MaxLength(1).eq_shallow(&Annotation::MaxLength(2)));
        assert!(
            Annotation::Index(None).eq_shallow(&Annotation::Index(Some(IndexValue {
                priority: None,
                name: "foo".to_string()
            })))
        );
    }
}

/// Represents a foreign key
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ForeignKey {
    /// Name of the table that should be referenced
    pub table_name: String,
    /// Name of the column that should be referenced
    pub column_name: String,
    /// Action to be used in case of on delete
    pub on_delete: ReferentialAction,
    /// Action to be used in case of an update
    pub on_update: ReferentialAction,
}

/**
Action that gets trigger on update and on delete.
*/
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ReferentialAction {
    /// Stop operation if any keys still depend on the parent table
    Restrict,
    /// The action is cascaded
    Cascade,
    /// The field is set to null
    SetNull,
    /// The field is set to its default
    SetDefault,
}

impl Default for ReferentialAction {
    fn default() -> Self {
        Self::Restrict
    }
}

impl Display for ReferentialAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferentialAction::Restrict => write!(f, "RESTRICT"),
            ReferentialAction::Cascade => write!(f, "CASCADE"),
            ReferentialAction::SetNull => write!(f, "SET NULL"),
            ReferentialAction::SetDefault => write!(f, "SET DEFAULT"),
        }
    }
}

/// Represents a complex index
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct IndexValue {
    /// Name of the index. Can be used multiple times in a [Model] to create an
    /// index with multiple columns.
    pub name: String,

    /// The order to put the columns in while generating an index.
    /// Only useful if multiple columns with the same name are present.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// A column's default value which is any non object / array json value
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum DefaultValue {
    /// Use hexadecimal to represent binary data
    String(String),
    /// i64 is used as it can represent any integer defined in DbType
    Integer(i64),
    /// Ordered float is used as f64 does not Eq and Order which are needed for Hash
    Float(OrderedFloat<f64>),
    /// Just a bool. Nothing interesting here.
    Boolean(bool),
}

/**
This implementation exists for strum::EnumIter
*/
impl Default for DefaultValue {
    fn default() -> Self {
        DefaultValue::Boolean(true)
    }
}
